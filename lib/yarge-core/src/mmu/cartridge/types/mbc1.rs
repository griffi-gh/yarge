use crate::Res;
use super::CartridgeImpl;

#[repr(u8)]
pub enum Mbc1Type {
  None, Ram, RamBattery
}
pub struct CartridgeMbc1 {
  rom: Vec<u8>,
  mbc1_type: Mbc1Type
}
impl CartridgeMbc1 {
  pub fn new(mbc1_type: Mbc1Type) -> Self {
    Self {
      rom: Vec::with_capacity(0x8000),
      mbc1_type
    }
  }
}
impl CartridgeImpl for CartridgeMbc1 {
  fn name(&self) -> &str { "MBC1" }
  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    self.rom.clear();
    self.rom.extend_from_slice(rom);
    self.rom.shrink_to_fit();
    Ok(())
  }
  fn read_rom(&self, _addr: u16) -> u8 {
    todo!()
  }
}
