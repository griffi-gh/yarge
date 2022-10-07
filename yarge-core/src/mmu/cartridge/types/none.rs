use crate::Res;
use crate::consts::ROM_SIZE;
use super::{CartridgeImpl, helpers::load_rom_static};

pub struct CartridgeNone {
  rom: Box<[u8; ROM_SIZE]>,
}
impl CartridgeNone {
  pub fn new() -> Self {
    Self {
      rom: Box::new([0xFF; ROM_SIZE]),
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
