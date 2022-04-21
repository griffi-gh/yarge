//TODO MBC3
use super::{
  common::{eram_addr, rom_addr, rom_bank_mask, eram_bank_mask},
  CartridgeImpl,
};

pub struct Type {
  timer: bool,
  ram: bool,
  battery: bool,
}

pub struct CartridgeMbc3 {
  mbc3_type: Type,
}
impl CartridgeMbc3 {
  pub fn new(mbc3_type: Type) -> Self {
    Self {
      mbc3_type
    }
  }
}
impl CartridgeImpl for CartridgeMbc3 {
  fn name(&self) -> &'static str { "MBC3" }
  fn read_rom(&self, addr: u16) -> u8 { 
    todo!()
  } 
}
