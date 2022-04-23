//TODO MBC3
use crate::Res;
use super::{
  common::{
    eram_addr, 
    rom_addr, 
    rom_bank_mask,
    eram_bank_mask,
    load_rom_vec,
  },
  header::RomHeader,
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
  ram_bank: u8,
}
impl CartridgeMbc3 {
  pub fn new(mbc3_type: Type, header: &RomHeader) -> Self {
    Self {
      mbc3_type,
      rom: Vec::with_capacity(0x8000),
      rom_bank: 1,
      ram_bank: 0,
    }
  }
}
impl CartridgeImpl for CartridgeMbc3 {
  fn name(&self) -> &'static str { "MBC3" }

  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    load_rom_vec(&mut self.rom, rom)
  }

  fn read_rom(&self, addr: u16) -> u8 { 
    if addr < 0x4000 {
      return self.rom[addr as usize];
    }
    return self.rom[rom_addr(addr, self.rom_bank)];
  } 

  fn write_rom(&mut self, _addr: u16, _value: u8) {
    todo!()
  }
}
