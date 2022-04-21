//TODO MBC3
use super::{
  common::{eram_addr, rom_addr, rom_bank_mask, eram_bank_mask},
  CartridgeImpl,
};

#[derive(Clone, Copy)]
pub struct Type {
  timer: bool,
  ram: bool,
  battery: bool,
}

pub struct CartridgeMbc3 {
  mbc3_type: Type,
  rom: Vec<u8>,
  rom_bank: u8,
}
impl CartridgeMbc3 {
  pub fn new(mbc3_type: Type) -> Self {
    Self {
      mbc3_type,
      rom: Vec::with_capacity(0x8000),
      rom_bank: 1,
    }
  }
}
impl CartridgeImpl for CartridgeMbc3 {
  fn name(&self) -> &'static str { "MBC3" }
  fn read_rom(&self, addr: u16) -> u8 { 
    if addr < 0x4000 {
      return self.rom[addr as usize];
    }
    return self.rom[rom_addr(addr, self.rom_bank)];
  } 
}
