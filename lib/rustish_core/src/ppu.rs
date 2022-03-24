mod oam;
mod ppu_registers;
mod bg_fetcher;
use bg_fetcher::BgFetcher;
use oam::OAMMemory;
use ppu_registers::LCDC;
use arraydeque::ArrayDeque;

#[derive(Clone, Copy, PartialEq, FromPrimitive, ToPrimitive)]
pub enum PPUMode {
  HBlank = 0,
  VBlank = 1,
  OAM    = 2,
  VRAM   = 3,
}
impl Default for PPUMode {
  fn default() -> Self { Self::HBlank }
}

pub struct PPU {
  mode: PPUMode,
  vram: [u8; 0x2000],
  oam: OAMMemory,
  bg_fetcher: BgFetcher,
  bg_fifo: ArrayDeque<[u8; 16]>,
  pub lcdc: LCDC,
  pub ly: u8,
}
impl PPU {
  pub fn new() -> Self {
    Self {
      mode: PPUMode::default(),
      vram: [0; 0x2000],
      oam: OAMMemory::new(),
      bg_fetcher: BgFetcher::new(),
      bg_fifo: ArrayDeque::default(),
      lcdc: LCDC::default(),
      ly: 0,
    }
  }
  pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.oam.write_oam(addr, value);
  }
  pub fn read_oam(&self, addr: u16) -> u8 {
    self.oam.read_oam(addr)
  }
  pub fn write_vram(&mut self, addr: u16, value: u8) {
    self.vram[(addr & 0x1FFF) as usize] = value;
  }
  pub fn read_vram(&self, addr: u16) -> u8 {
    self.vram[(addr & 0x1FFF) as usize]
  }
  pub fn tick(&self) {
    //TODO ppu.tick()
  }
}