use crate::{Res, YargeError};
use super::CartridgeImpl;

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
    if rom.len() != 0x8000 {
      return Err(
        YargeError::InvalidRomSize(rom.len())
      );
    }
    for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
      *place = *data;
    }
    Ok(())
  }

  fn read_rom(&self, addr: u16) -> u8 {
    return self.rom[(addr & 0x7FFF) as usize];
  }
}
