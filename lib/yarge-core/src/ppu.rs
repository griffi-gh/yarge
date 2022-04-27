mod oam;
mod ppu_registers;
mod fetcher;
use fetcher::{Fetcher, FetcherLayer, FifoPixel};
use oam::OamMemory;
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
  ly: u8,
  lx: u8, 
  hblank_len: usize,
  cycles: usize,
  mode: PpuMode,
  vram: Box<[u8; VRAM_SIZE]>,
  oam: OamMemory,
  lcdc: Lcdc,
  display_cleared: bool,
  bg_fetcher: Fetcher,
  to_discard: u8,
  stat_intr: StatInterrupts, 
  stat_prev: bool,
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
      ly: 0,
      lx: 0,
      hblank_len: 204,
      cycles: 0,
      mode: PpuMode::default(),
      vram: Box::new([0; VRAM_SIZE]),
      oam: OamMemory::new(),
      lcdc: Lcdc::default(),
      display_cleared: false,
      bg_fetcher: Fetcher::new(),
      to_discard: 0,
      stat_intr: StatInterrupts::default(),
      stat_prev: false,
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
    (self.mode as u8) | 
    ((self.ly == self.lyc) as u8) << 2 |
    (self.stat_intr.into_u8() << 3)
  }
  pub fn set_stat(&mut self, value: u8) {
    self.stat_intr.set_from_u8(value >> 3);
  }

  //TODO check for mode 2 and 3
  pub fn read_oam(&self, addr: u16, blocking: bool) -> u8 {
    self.oam.read_oam(addr - 0xFE00)
  }
  pub fn write_oam(&mut self, addr: u16, value: u8, blocking: bool) {
    self.oam.write_oam(addr - 0xFE00, value);
  }

  //TODO check for mode 3
  pub fn read_vram(&self, addr: u16, blocking: bool) -> u8 {
    self.vram[(addr - 0x8000) as usize]
  }
  pub fn write_vram(&mut self, addr: u16, value: u8, blocking: bool) {
    self.vram[(addr - 0x8000) as usize] = value;
  }
  
  fn mode(&mut self, mode: PpuMode) {
    self.cycles = 0;
    self.mode = mode;
  }

  fn check_stat(&mut self, iif: &mut u8) {
    let stat = {
      (self.stat_intr.lyc_eq && (self.ly == self.lyc)) ||
      (self.stat_intr.mode_0 && (self.mode as u8 == 0)) ||
      (self.stat_intr.mode_1 && (self.mode as u8 == 1)) ||
      (self.stat_intr.mode_2 && (self.mode as u8 == 2))
    };
    if stat && !self.stat_prev {
      Cpu::set_interrupt(iif, Interrupt::Stat);
    }
    self.stat_prev = stat;
  }

  fn tick_inner(&mut self, iif: &mut u8) {
    if !self.lcdc.enable_display {
      if !self.display_cleared {
        //TODO find out exact values
        *self.display = [0; FB_SIZE];
        self.ly = 0;
        self.lx = 0;
        self.stat_prev = false;
        self.mode(PpuMode::OamSearch); //resets cycles too
        self.display_cleared = true;
      }
      return;
    } else {
      self.display_cleared = false;
    }
    match self.mode { 
      PpuMode::HBlank => {
        if self.cycles >= self.hblank_len {
          self.ly += 1;
          if self.ly >= 144 {
            self.mode(PpuMode::VBlank);
            self.frame_ready = true;
            Cpu::set_interrupt(iif, Interrupt::VBlank);
          } else {
            self.mode(PpuMode::OamSearch);
          }
          self.check_stat(iif);
        }
      },
      PpuMode::VBlank => {
        if self.cycles >= 456 {
          self.cycles = 0;
          self.ly += 1;
          if self.ly >= 155 {
            self.ly = 0;
            self.mode(PpuMode::OamSearch);
          }
          self.check_stat(iif);
        }
      },
      PpuMode::OamSearch => {
        //TODO
        if self.cycles >= 80 {
          self.bg_fetcher.start(self.scx, self.scy, self.ly, FetcherLayer::Background);
          self.to_discard = self.scx & 7;
          self.mode(PpuMode::PxTransfer);
          self.check_stat(iif);
        }
      },
      PpuMode::PxTransfer => {
        //TODO check for bg enable
        self.bg_fetcher.tick(&self.lcdc, &self.vram);
        if self.bg_fetcher.len() > 0 {
          let FifoPixel { color, .. } = self.bg_fetcher.pop().unwrap();
          if self.to_discard > 0 {
            self.to_discard -= 1;
          }
          if self.to_discard == 0 {
            let addr = (self.ly as usize * WIDTH) + self.lx as usize;
            self.display[addr] = (self.bgp >> (color << 1)) & 0b11;
            self.lx += 1;
            if self.lx >= WIDTH as u8 { 
              self.lx = 0;
              #[cfg(debug_assertions)] {
                assert!(self.cycles >= 172, "PxTransfer took less then 172 cycles: {}", self.cycles);
                assert!(self.cycles <= 289, "PxTransfer took more then 289 cycles: {}", self.cycles);
              }
              self.hblank_len = 376 - self.cycles;
              self.mode(PpuMode::HBlank);
              self.check_stat(iif);
            }
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
