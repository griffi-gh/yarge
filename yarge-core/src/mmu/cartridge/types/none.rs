use crate::Res;
use super::{CartridgeImpl, helpers::load_rom_static};

pub struct CartridgeNone {
  rom: Box<[u8; 0x8000]>,
}
impl CartridgeNone {
  pub fn new() -> Self {
    Self {
      rom: Box::new([0xFF; 0x8000]),
    }
  }
}
impl CartridgeImpl for CartridgeNone {
  fn name(&self) -> &'static str { "ROM ONLY" }
  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    load_rom_static(&mut self.rom, rom)
  }
  fn read_rom(&self, addr: u16) -> u8 {
    self.rom[(addr & 0x7FFF) as usize]
  }
}
