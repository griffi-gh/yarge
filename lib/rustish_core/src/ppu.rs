pub mod oam;
pub mod ppu_registers;
use oam::OAMMemory;
use ppu_registers::LCDC;
use arraydeque::ArrayDeque;

#[derive(Clone, Copy, PartialEq)]
pub enum PPUMode {
  HBlank = 0_u8,
  VBlank = 1_u8,
  OAM    = 2_u8,
  VRAM   = 3_u8,
}
impl Default for PPUMode {
  fn default() -> Self { Self::HBlank }
}

pub struct PPU {
  vram: [u8; 0x2000],
  oam: OAMMemory,
  pub lcdc: LCDC,
  pub ly: u8,
}
impl PPU {
  pub fn new() -> Self {
    Self {
      lcdc: LCDC::default(),
      vram: [0; 0x2000],
      oam: OAMMemory::new(),
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
  pub fn tick(&self, _t: u32) {
    //TODO
  }
}