mod oam;
mod ppu_registers;
mod fetcher;
use fetcher::{Fetcher, FetcherLayer, FifoPixel};
use oam::OAMMemory;
use ppu_registers::{LCDC, PPUMode};
use crate::consts::{VRAM_MAX, VRAM_SIZE, WIDTH, FB_SIZE};

pub struct PPU {
  pub display: [u8; FB_SIZE],
  pub ly: u8,
  cycles: usize,
  x: u8,
  mode: PPUMode,
  vram: [u8; 0x2000],
  oam: OAMMemory,
  lcdc: LCDC,
  bg_fetcher: Fetcher,
}
impl PPU {
  pub fn new() -> Self {
    Self {
      display: {
        let mut display = [0; FB_SIZE];
        //fill display with fancy-ass pattern
        for i in 0..FB_SIZE {
          display[i] = (((i + (i / WIDTH)) & 1) as u8) * (1 + (i % 3) as u8);
        }
        display
      },
      ly: 0,
      cycles: 0,
      x: 0,
      mode: PPUMode::HBlank,
      vram: [0; VRAM_SIZE],
      oam: OAMMemory::new(),
      lcdc: LCDC::default(),
      bg_fetcher: Fetcher::new(FetcherLayer::Background),
    }
  }

  #[inline] pub fn set_lcdc(&mut self, value: u8) {
    self.lcdc.set_from_u8(value);
  }
  #[inline] pub fn get_lcdc(&self) -> u8 {
    self.lcdc.into_u8()
  }

  #[inline] pub fn read_oam(&self, addr: u16) -> u8 {
    self.oam.read_oam(addr - 0xFE00)
  }
  #[inline] pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.oam.write_oam(addr - 0xFE00, value);
  }

  #[inline] pub fn read_vram(&self, addr: u16) -> u8 {
    self.vram[(addr & VRAM_MAX) as usize]
  }
  #[inline] pub fn write_vram(&mut self, addr: u16, value: u8) {
    self.vram[(addr & VRAM_MAX) as usize] = value;
  }
  
  fn mode(&mut self, mode: PPUMode) {
    self.cycles = 0;
    self.mode = mode;
  }

  pub fn tick(&mut self) {
    self.cycles += 4;
    match self.mode { 
      PPUMode::HBlank => {
        self.ly += 1;
        if self.ly >= 144 {
          self.mode(PPUMode::VBlank);
        }
      },
      PPUMode::VBlank => {
        self.cycles = 0;
        self.ly = 0;
        self.mode(PPUMode::OamSearch);
      },
      PPUMode::OamSearch => {
        //TODO
        if self.cycles >= 160 {
          self.mode(PPUMode::PxTransfer);
        }
      },
      PPUMode::PxTransfer => {
        self.bg_fetcher.tick(&self.lcdc, &self.vram);
        if self.bg_fetcher.len() >= 8 {
          let FifoPixel { color, .. } = self.bg_fetcher.pop().unwrap();
          self.display[(self.ly as usize * WIDTH) + self.x as usize] = color;
          self.x += 1;
          if self.x >= WIDTH as u8 { 
            self.x = 0;
            self.mode(PPUMode::HBlank);
          }
        }
      }
    }
  }
}
