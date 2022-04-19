use crate::Res;
use super::CartridgeImpl;

#[repr(u8)]
#[derive(PartialEq)]
pub enum Mbc1Type {
  None, Ram, RamBattery
}

pub struct CartridgeMbc1 {
  rom: Vec<u8>,
  eram: Option<Vec<u8>>,
  rom_bank: u8,
  ram_bank: u8,
  mbc1_type: Mbc1Type,
}
impl CartridgeMbc1 {
  pub fn new(mbc1_type: Mbc1Type) -> Self {
    Self {
      rom: Vec::with_capacity(0x8000),
      eram: None,
      rom_bank: 1,
      ram_bank: 0,
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
  fn read_rom(&self, addr: u16) -> u8 {
    if addr < 0x4000 {
      return self.rom[addr as usize];
    }
    todo!()
  }
  fn write_rom(&mut self, addr: u16, value: u8) {
    todo!()
  }
}
