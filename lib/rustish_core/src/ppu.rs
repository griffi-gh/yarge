mod oam;
mod ppu_registers;
mod fetcher;
use fetcher::{Fetcher, FetcherLayer};
use oam::OAMMemory;
use ppu_registers::{LCDC, PPUMode};

pub const VRAM_SIZE: usize = 0x2000;
pub const VRAM_MAX: u16 = (VRAM_SIZE - 1) as u16;
pub const WIDTH: usize = 160;
pub const HEIGHT: usize = 144;

pub struct PPU {
  pub display: [u8; WIDTH * HEIGHT],
  pub ly: u8,
  mode: PPUMode,
  vram: [u8; 0x2000],
  oam: OAMMemory,
  lcdc: LCDC,
  bg_fetcher: Fetcher,
}
impl PPU {
  pub fn new() -> Self {
    Self {
      /*pub*/ display: [0; WIDTH * HEIGHT],
      /*pub*/ ly: 0,
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
  
  pub fn tick(&mut self) {
    //TODO ppu.tick()
  }
}
