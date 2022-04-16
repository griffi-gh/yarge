mod oam;
mod ppu_registers;
mod fetcher;
use fetcher::{Fetcher, FetcherLayer, FifoPixel};
use oam::OAMMemory;
use ppu_registers::{LCDC, PPUMode};
use crate::consts::{VRAM_SIZE, WIDTH, FB_SIZE};

pub struct PPU {
  pub display: Box<[u8; FB_SIZE]>,
  pub scy: u8,
  pub scx: u8,
  pub frame_ready: bool,
  ly: u8, x: u8, 
  hblank_len: usize,
  cycles: usize,
  mode: PPUMode,
  vram: Box<[u8; VRAM_SIZE]>,
  oam: OAMMemory,
  lcdc: LCDC,
  bg_fetcher: Fetcher,
}
impl PPU {
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
      scy: 0, scx: 0,
      frame_ready: false,
      ly: 0, x: 0,
      hblank_len: 204,
      cycles: 0,
      mode: PPUMode::default(),
      vram: Box::new([0; VRAM_SIZE]),
      oam: OAMMemory::new(),
      lcdc: LCDC::default(),
      bg_fetcher: Fetcher::new(),
    }
  }

  #[inline] pub fn get_ly(&self) -> u8 { self.ly }

  #[inline] pub fn set_lcdc(&mut self, value: u8) {
    self.lcdc.set_from_u8(value);
  }
  #[inline] pub fn get_lcdc(&self) -> u8 {
    self.lcdc.into_u8()
  }

  pub fn get_stat(&self) -> u8 {
    self.mode as u8
  }
  pub fn set_stat(&mut self, _value: u8) {
    //TODO set_stat()
  }

  #[inline] pub fn read_oam(&self, addr: u16) -> u8 {
    self.oam.read_oam(addr - 0xFE00)
  }
  #[inline] pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.oam.write_oam(addr - 0xFE00, value);
  }

  #[inline] pub fn read_vram(&self, addr: u16) -> u8 {
    self.vram[(addr - 0x8000) as usize]
  }
  #[inline] pub fn write_vram(&mut self, addr: u16, value: u8) {
    self.vram[(addr - 0x8000) as usize] = value;
  }
  
  fn mode(&mut self, mode: PPUMode) {
    self.cycles = 0;
    self.mode = mode;
  }

  pub fn tick(&mut self) {
    self.cycles += 4;
    match self.mode { 
      PPUMode::HBlank => {
        if self.cycles >= self.hblank_len {
          self.ly += 1;
          if self.ly >= 144 {
            self.frame_ready = true;
            self.mode(PPUMode::VBlank);
          } else {
            self.mode(PPUMode::OamSearch);
          }
        }
      },
      PPUMode::VBlank => {
        if self.cycles >= 456 {
          self.cycles = 0;
          self.ly += 1;
          if self.ly >= 155 {
            self.ly = 0;
            self.mode(PPUMode::OamSearch);
          }
        }
      },
      PPUMode::OamSearch => {
        //TODO
        if self.cycles >= 80 {
          self.bg_fetcher.start(self.scx, self.scy, self.ly, FetcherLayer::Background);
          self.mode(PPUMode::PxTransfer);
        }
      },
      PPUMode::PxTransfer => {
        //TODO check for scx
        self.bg_fetcher.tick(&self.lcdc, &self.vram);
        if self.bg_fetcher.len() > 8 {
          let FifoPixel { color, .. } = self.bg_fetcher.pop().unwrap();
          let addr = (self.ly as usize * WIDTH) + self.x as usize;
          self.display[addr] = color;
          self.x += 1;
          if self.x >= WIDTH as u8 { 
            self.x = 0;
            #[cfg(debug_assertions)] {
              assert!(self.cycles >= 172, "PxTransfer took less then 172 cycles: {}", self.cycles);
              assert!(self.cycles <= 289, "PxTransfer took more then 289 cycles: {}", self.cycles);
            }
            self.hblank_len = 376 - self.cycles;
            self.mode(PPUMode::HBlank);
          }
        }
      }
    }
  }

  pub fn render_tileset(&self) {
    todo!() //TODO render_tileset
  }
}
