mod oam;
mod ppu_registers;
mod fifo;
mod util;

use fifo::{Fetcher, BackgroundFetcher, SpriteFetcher, FifoPixel};
use oam::{OamMemory, OamBuffer};
use ppu_registers::{Lcdc, PpuMode, StatInterrupts};
use crate::{
  consts::{VRAM_SIZE, WIDTH, FB_SIZE, OBJECTS_PER_LINE},
  cpu::{Cpu, Interrupt}
};

pub struct Ppu {
  pub display: Box<[u8; FB_SIZE]>,
  pub frame_ready: bool,
  pub bgp: u8,
  pub obp: (u8, u8),
  pub lyc: u8,
  pub scy: u8,
  pub scx: u8,
  pub wy: u8,
  pub wx: u8,
  wly: u8,
  ly: u8,
  pub mmio_ly: u8,
  compare_ly: u8,
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
  fetched_sprites: usize,
  pub mmu_oam_locked: bool,
  /*HACK*/ stat_r_lyc_eq: bool,
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
      obp: (0b11_10_01_00, 0b11_10_01_00),
      lyc: 0,
      scy: 0,
      scx: 0,
      wy: 0,
      wx: 0,
      wly: 0,
      ly: 0,
      mmio_ly: 0,
      compare_ly: 0,
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
      fetched_sprites: 0,
      mmu_oam_locked: false,
      stat_r_lyc_eq: false
    }
  }

  pub fn set_lcdc(&mut self, value: u8) {
    self.lcdc.set_from_u8(value);
  }
  pub fn get_lcdc(&self) -> u8 {
    self.lcdc.into_u8()
  }

  pub fn get_stat(&self) -> u8 {
    (self.lcdc.enable_display as u8 * self.mode as u8) | 
    ((self.stat_r_lyc_eq as u8) << 2) |
    (self.stat_intr.into_u8() << 3) |
    0x80
  }
  pub fn set_stat(&mut self, value: u8) {
    self.stat_intr.set_from_u8(value >> 3);
  }

  fn oam_blocked(&self) -> bool {
    #[cfg(feature = "dbg-ly-stub")] { return false }
    if !self.lcdc.enable_display { return false }
    if self.mmu_oam_locked { return true }
    matches!(self.mode, PpuMode::OamSearch | PpuMode::PxTransfer)
  }
  fn vram_blocked(&self) -> bool {
    #[cfg(feature = "dbg-ly-stub")] { return false }
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
    #[cfg(feature = "dbg-emit-ppu-events")] {
      println!("PPU_EVENT CHANGE_MODE mode={} from={} ly={}", mode as u8, self.mode as u8, self.ly);
    }
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

  fn do_sprite_fetcher_stuff(&mut self) {
    if !self.spr_fetcher.fetching {
      if let Some(sprite) = self.oam_buffer.get(self.fetched_sprites) {
        if sprite.x <= (self.lx + 8) { // is == ok here?
          //Initiate sprite fetch
          self.bg_fetcher.spr_reset();
          self.suspend_bg_fetcher = true;
          self.spr_fetcher.start(*sprite, self.ly);
          self.fetched_sprites += 1;
          debug_assert!(self.fetched_sprites <= OBJECTS_PER_LINE, "Fetched too much sprites");
          #[cfg(feature = "dbg-emit-ppu-events")] {
            println!("PPU_EVENT SPR_FETCH_START lx={} ly={} cycles={}", self.lx, self.ly, self.cycles);
          }
        }
      }
    }
    //Tick spr_fetcher if it's not done fetching stuff
    if self.spr_fetcher.fetching {
      #[cfg(feature = "dbg-emit-ppu-events")] let prev_state = self.spr_fetcher.state;
      self.spr_fetcher.tick(&self.lcdc, &self.vram);
      #[cfg(feature = "dbg-emit-ppu-events")] if prev_state != self.spr_fetcher.state {
        println!("PPU_EVENT SPR_FETCHER_STATE_CHANGE cycles={} ly={} next={} prev={}", self.cycles, self.ly, self.spr_fetcher.state as u8, prev_state as u8);
      }
    }
  }

  fn set_ly_and_update(&mut self, ly: u8) {
    self.ly = ly;
    self.mmio_ly = ly;
    self.compare_ly = ly;
    // Should I check STAT here instead?
    // self.check_stat(iif);
  }

  fn line_153_weirdness(&mut self) {
    //TODO line 153
    // match self.cycles {
    //   0 => {
    //     self.mmio_ly = 153;
    //     self.compare_ly = u8::MAX;
    //   },
    //   6 => {
    //     self.mmio_ly = 0;
    //     self.compare_ly = 153;
    //   },
    //   8 => {
    //     self.mmio_ly = 0;
    //     self.compare_ly = u8::MAX;
    //   },
    //   12 => {
    //     self.compare_ly = 0;
    //   },
    //   456 => {
        
    //   },
    //   _ => {}
    // }
  }

  fn tick_inner(&mut self, iif: &mut u8) {
    if !self.lcdc.enable_display {
      if !self.display_cleared {
        *self.display = [0; FB_SIZE];
        //TODO finish fixing mrdo
        //reset ly start
        //println!("comparison ly {} => {}, then mmio/ly reset", self.compare_ly, self.ly);
        self.set_ly_and_update(0);
        //reset ly end
        self.lx = 0;
        self.wly = 0;
        self.stat_prev = false;
        self.mode(PpuMode::HBlank); //<= resets cycles too
        //self.mode(PpuMode::OamSearch);  
        self.display_cleared = true;
      }
      return
    } else {
      self.stat_r_lyc_eq = self.compare_ly == self.lyc;
      self.display_cleared = false;
    }
    match self.mode { 
      PpuMode::HBlank => {
        if self.cycles >= self.hblank_len {
          self.set_ly_and_update(self.ly + 1);
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
        if self.ly == 153 {
          self.line_153_weirdness();
        }
        if self.cycles >= 456 {
          self.cycles = 0;
          self.set_ly_and_update(self.ly + 1);
          if self.ly >= 155 {
            #[cfg(feature = "dbg-emit-ppu-events")] {
              println!("PPU_EVENT FRAME_END");
            }
            self.wly = 0;
            self.set_ly_and_update(0);
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
          } else {
            self.oam_buffer = OamBuffer::default();
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
      PpuMode::PxTransfer => { //This is probably extremely inaccurate!
        let mut push_color: Option<u8> = None;

        //TODO implement sprite fetch abort
        if (self.oam_buffer.len() != 0) && self.lcdc.enable_obj {
          //Check for sprite fetch
          self.do_sprite_fetcher_stuff();

          //Un-suspend bg fetcher if the sprite fetcher is done fetching the sprite
          if self.suspend_bg_fetcher && !self.spr_fetcher.fetching {
            #[cfg(feature = "dbg-emit-ppu-events")] {
              println!("PPU_EVENT SPR_FETCH_END lx={} ly={} cycles={}", self.lx, self.ly, self.cycles);
            }
            self.suspend_bg_fetcher = false;
            //Re-check In case two sprites overlap
            self.do_sprite_fetcher_stuff(); 
            // Mayyybee I even need this check
            // if !self.spr_fetcher.fetching {
            //   self.suspend_bg_fetcher = false;
            // }
          }
        }

        //Update bg fetcher if not suspended
        if !self.suspend_bg_fetcher {
          //Update values
          self.bg_fetcher.update_values(self.scx, self.scy);
          //Switch to window if the pixel is in window
          //TODO: line 166 bug emulation is causing missing bg
          //if !self.bg_fetcher.is_window() && self.window_in_ly() && (((self.lx + 7) >= self.wx) || (self.wx == 166)) {
          if !self.bg_fetcher.is_window() && self.window_in_ly() && ((self.lx + 7) >= self.wx) {
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

        //Pixel mixing
        let spr_pixel = if (self.spr_fetcher.len() > 0) && push_color.is_some() {
          let spr_pixel = self.spr_fetcher.pop().unwrap();
          if
            (spr_pixel.color > 0) && 
            (!spr_pixel.priority || (push_color.unwrap() == 0))
          {
            push_color = Some(spr_pixel.color);
            Some(spr_pixel)
          } else { None }
        } else { None };

        //Map to pal
        push_color = if let Some(color) = push_color {
          let pal = if let Some(pixel) = spr_pixel {
            let pal = if !pixel.pal { self.obp.0 } else { self.obp.1 };
            pal & 0b11111100 //Index 0 is always transparent
          } else {
            self.bgp
          };
          Some((pal >> (color << 1)) & 0b11)
        } else { None };

        //Push pixel to the display
        if let Some(color) = push_color {
          //Get display addr and set pixel color
          let addr = (self.ly as usize * WIDTH) + self.lx as usize;
          self.display[addr] = color;
          //Move to the next pixel
          self.lx += 1;
          #[cfg(feature = "dbg-emit-ppu-events")] {
            println!("PPU_EVENT LX_INC lx={} ly={} cycles={}", self.lx, self.ly, self.cycles);
          }
          //End PxTransfer if lx > WIDTH
          if self.lx >= WIDTH as u8 { 
            #[cfg(feature = "dbg-emit-ppu-events")] {
              println!("PPU_EVENT PX_FETCH_LINE_END ly={} cycles={}", self.ly, self.cycles);
            }
            #[cfg(debug_assertions)]
            if self.fetched_sprites != self.oam_buffer.len() {
              eprintln!("Fetched {} sprites out of {}", self.fetched_sprites, self.oam_buffer.len());
            }
            debug_assert!(self.cycles >= 172, "PxTransfer took less then 172 cycles: {}", self.cycles);
            debug_assert!(self.cycles <= 289, "PxTransfer took more then 289 cycles: {}", self.cycles);
            self.fetched_sprites = 0;
            self.spr_fetcher.eol();
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
      PpuMode::OamSearch => {
        self.cycles += 4;
        self.tick_inner(iif);
      },
      PpuMode::VBlank if self.ly != 153 => {
        self.cycles += 4;
        self.tick_inner(iif);
      },
      _ => {
        for _ in 0..4 {
          self.cycles += 1;
          self.tick_inner(iif);
        }
      }
    }
  }

  pub fn render_tileset(&self) {
    todo!() //TODO render_tileset
  }
}
impl Default for Ppu {
  fn default() -> Self {
    Self::new()
  }
}
