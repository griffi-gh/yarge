mod oam;
mod ppu_registers;
mod fifo;
mod util;

use fifo::{Fifo, BackgroundFetcher, SpriteFetcher, FifoPixel};
use oam::{OamMemory, OamBuffer};
use ppu_registers::{Lcdc, PpuMode, StatInterrupts};
use crate::{
  consts::{VRAM_SIZE, WIDTH, FB_SIZE},
  cpu::{Cpu, Interrupt}
};

pub struct Ppu {
  pub display: Box<[u8; FB_SIZE]>,
  pub frame_ready: bool,
  pub bgp: u8,
  pub lyc: u8,
  pub scy: u8,
  pub scx: u8,
  pub wy: u8,
  pub wx: u8,
  wly: u8,
  ly: u8,
  lx: u8, 
  hblank_len: usize,
  cycles: usize,
  mode: PpuMode,
  vram: Box<[u8; VRAM_SIZE]>,
  oam: OamMemory,
  lcdc: Lcdc,
  display_cleared: bool,
  bg_fetcher: BackgroundFetcher,
  spr_fetcher: SpriteFetcher,
  to_discard: u8,
  stat_intr: StatInterrupts, 
  stat_prev: bool,
  oam_buffer: OamBuffer,
  suspend_bg_fetcher: bool,
}
impl Ppu {
  pub fn new() -> Self {
    Self {
      display: {
        let mut display = Box::new([0; FB_SIZE]);
        //fill display with fancy-ass pattern
        for i in 0..FB_SIZE {
          display[i] = (((i + (i / WIDTH)) & 1) as u8) * (1 + (i % 3) as u8);
        }
        display
      },
      frame_ready: false,
      bgp: 0b11_10_01_00,
      lyc: 0,
      scy: 0,
      scx: 0,
      wy: 0,
      wx: 0,
      wly: 0,
      ly: 0,
      lx: 0,
      hblank_len: 204,
      cycles: 0,
      mode: PpuMode::default(),
      vram: Box::new([0; VRAM_SIZE]),
      oam: OamMemory::new(),
      lcdc: Lcdc::default(),
      display_cleared: false,
      bg_fetcher: BackgroundFetcher::new(),
      spr_fetcher: SpriteFetcher::new(),
      to_discard: 0,
      stat_intr: StatInterrupts::default(),
      stat_prev: false,
      oam_buffer: OamBuffer::default(),
      suspend_bg_fetcher: false,
    }
  }

  pub fn get_ly(&self) -> u8 { self.ly }

  pub fn set_lcdc(&mut self, value: u8) {
    self.lcdc.set_from_u8(value);
  }
  pub fn get_lcdc(&self) -> u8 {
    self.lcdc.into_u8()
  }

  pub fn get_stat(&self) -> u8 {
    (self.lcdc.enable_display as u8 * self.mode as u8) | 
    (((self.ly == self.lyc) as u8) << 2) |
    (self.stat_intr.into_u8() << 3) |
    0x80
  }
  pub fn set_stat(&mut self, value: u8) {
    self.stat_intr.set_from_u8(value >> 3);
  }

  fn oam_blocked(&self) -> bool {
    #[cfg(feature = "ly-stub")]  { return false }
    if !self.lcdc.enable_display { return false }
    matches!(self.mode, PpuMode::OamSearch | PpuMode::PxTransfer)
  }
  fn vram_blocked(&self) -> bool {
    #[cfg(feature = "ly-stub")] { return false }
    self.mode == PpuMode::PxTransfer
  }

  pub fn read_oam(&self, addr: u16, blocking: bool) -> u8 {
    if blocking && self.oam_blocked() { return 0xff }
    self.oam.read_oam(addr - 0xFE00)
  }
  pub fn write_oam(&mut self, addr: u16, value: u8, blocking: bool) {
    if blocking && self.oam_blocked() { return }
    self.oam.write_oam(addr - 0xFE00, value);
  }

  pub fn read_vram(&self, addr: u16, blocking: bool) -> u8 {
    if blocking && self.vram_blocked() { return 0xFF }
    self.vram[(addr - 0x8000) as usize]
  }
  pub fn write_vram(&mut self, addr: u16, value: u8, blocking: bool) {
    if blocking && self.vram_blocked() { return }
    self.vram[(addr - 0x8000) as usize] = value;
  }
  
  fn mode(&mut self, mode: PpuMode) {
    self.cycles = 0;
    self.mode = mode;
  }

  fn check_stat(&mut self, iif: &mut u8) {
    let stat = {
      (self.stat_intr.lyc_eq && (self.ly == self.lyc)) ||
      (self.stat_intr.mode_0 && (self.mode == PpuMode::HBlank)) ||
      (self.stat_intr.mode_1 && (self.mode == PpuMode::VBlank)) ||
      (self.stat_intr.mode_2 && (self.mode == PpuMode::OamSearch))
    };
    if stat && !self.stat_prev {
      Cpu::set_interrupt(iif, Interrupt::Stat);
    }
    self.stat_prev = stat;
  }

  fn window_in_ly(&self) -> bool {
    self.lcdc.enable_win && 
    (self.ly >= self.wy) && 
    (self.wx < (WIDTH + 7) as u8) || (self.wx == 166)
  }

  fn tick_inner(&mut self, iif: &mut u8) {
    if !self.lcdc.enable_display {
      if self.display_cleared {
        return;
      }
      //TODO find out exact values
      *self.display = [0; FB_SIZE];
      self.ly = 0;
      self.lx = 0;
      self.wly = 0;
      self.stat_prev = false;
      self.mode(PpuMode::OamSearch); //resets cycles too
      self.display_cleared = true;
    } else {
      self.display_cleared = false;
    }
    match self.mode { 
      PpuMode::HBlank => {
        if self.cycles >= self.hblank_len {
          self.ly += 1;
          if self.ly < 144 {
            self.mode(PpuMode::OamSearch);
          } else {
            self.mode(PpuMode::VBlank);
            self.frame_ready = true;
            Cpu::set_interrupt(iif, Interrupt::VBlank);
          }
          self.check_stat(iif);
        }
      },
      PpuMode::VBlank => {
        if self.cycles >= 456 {
          self.cycles = 0;
          self.ly += 1;
          if self.ly >= 155 {
            self.wly = 0;
            self.ly = 0;
            self.mode(PpuMode::OamSearch);
          }
          self.check_stat(iif);
        }
      },
      PpuMode::OamSearch => {
        if self.cycles >= 80 {
          //TODO verify if doing it all at once is ok
          if self.lcdc.enable_obj {
            self.oam_buffer = self.oam.get_buffer(self.ly, &self.lcdc);
          }
          self.to_discard = self.scx & 7;
          self.bg_fetcher.start(
            self.scx, self.scy,
            self.ly, self.wly
          );
          if !self.bg_fetcher.is_window() && self.window_in_ly() && self.wx <= 7 {
            self.bg_fetcher.switch_to_window();
          }
          self.mode(PpuMode::PxTransfer);
          self.check_stat(iif);
        }
      },
      PpuMode::PxTransfer => { 
        //This is probably inaccurate!
        //TODO optimize if bg/win is off

        let mut push_color: Option<u8> = None;

        //Update values
        self.bg_fetcher.update_values(self.scx, self.scy);

        // TODO re-enable
        // //Check for sprite fetch
        // //need to check if the spr fetcher is running???
        // for sprite_idx in 0..self.oam_buffer.len() {
        //   let sprite = self.oam_buffer.get(sprite_idx).unwrap();
        //   if sprite.x <= (self.lx + 8) {
        //     //Initiate sprite fetch
        //     self.bg_fetcher.spr_reset();
        //     self.suspend_bg_fetcher = true;
        //     break;
        //   }
        // }

        //Update bg fetcher if not fetching sprites
        //Otherwise its sus pended
        if !self.suspend_bg_fetcher {
          //Switch to window if the pixel is in window
          if !self.bg_fetcher.is_window() && self.window_in_ly() && (((self.lx + 7) >= self.wx) || (self.wx == 166)) {
            self.bg_fetcher.switch_to_window();
          }
          self.bg_fetcher.tick(&self.lcdc, &self.vram);
          //If bg fetcher has something
          if self.bg_fetcher.len() > 0 {
            //Shift out background pixel
            let FifoPixel { color, .. } = self.bg_fetcher.pop().unwrap();
            //Discard bg pixel if needed
            if !self.bg_fetcher.is_window() && self.to_discard > 0 {
              self.to_discard -= 1;
            } else {
              //Set color to 0 if bg is disabled
              if !self.lcdc.enable_bg {
                push_color = Some(0);
              } else {
                push_color = Some(color);
              }
            }
          }
        }

        //TODO REMOVE: CHANGE PIXEL COLOR IF IT HAS A SPRITE
        if push_color.is_some() {
          for sprite_idx in 0..self.oam_buffer.len() {
            let sprite = self.oam_buffer.get(sprite_idx).unwrap();
            if (sprite.x <= (self.lx + 8)) && (sprite.x > self.lx) {
              push_color = Some((push_color.unwrap() + 1) % 4);
              break
            }
          }
        }

        //Push pixel to the display
        if let Some(color) = push_color {
          //Get display addr and set pixel color
          let addr = (self.ly as usize * WIDTH) + self.lx as usize;
          self.display[addr] = (self.bgp >> (color << 1)) & 0b11;
          //Move to the next pixel
          self.lx += 1;
          //End PxTransfer if lx > WIDTH
          if self.lx >= WIDTH as u8 { 
            debug_assert!(self.cycles >= 172, "PxTransfer took less then 172 cycles: {}", self.cycles);
            debug_assert!(self.cycles <= 289, "PxTransfer took more then 289 cycles: {}", self.cycles);
            self.lx = 0;
            self.hblank_len = 376 - self.cycles;
            if self.window_in_ly() {
              self.wly += 1;
            }
            self.mode(PpuMode::HBlank);
            self.check_stat(iif);
          }
        }

      }
    }
  }

  pub fn tick(&mut self, iif: &mut u8) {
    //TODO optimize waits
    match self.mode {
      PpuMode::PxTransfer | PpuMode::HBlank => {
        for _ in 0..4 {
          self.cycles += 1;
          self.tick_inner(iif);
        }
      },
      PpuMode::VBlank | PpuMode::OamSearch => {
        self.cycles += 4;
        self.tick_inner(iif);
      }
    }
  }

  pub fn render_tileset(&self) {
    todo!() //TODO render_tileset
  }
}
