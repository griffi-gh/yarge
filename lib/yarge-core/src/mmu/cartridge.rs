use enum_dispatch::enum_dispatch;
use crate::{Res, YargeError};

mod types;
pub use types::*;

mod header;
pub use header::RomHeader;

#[enum_dispatch]
#[allow(unused_variables)]
pub trait CartridgeImpl {
  fn name(&self) -> &str;
  fn load_rom(&mut self, data: &[u8]) -> Res<()> { Ok(()) }
  fn read_rom(&self, addr: u16) -> u8;
  fn write_rom(&mut self, addr: u16, value: u8) { }
  fn read_eram(&self, addr: u16) -> u8 { 0xff }
  fn write_eram(&self, addr: u16, value: u8) {}
  fn save_eram(&self) -> Option<Vec<u8>> { None }
}

#[non_exhaustive]
#[enum_dispatch(CartridgeImpl)]
pub enum Cartridge {
  MockCartridge,
  CartridgeNone,
  CartridgeMbc1,
}

pub fn get_cartridge(cart_type: u8) -> Res<Cartridge> {
  match cart_type {
    0x00 => Ok(CartridgeNone::new().into()),
    0x01 => Ok(CartridgeMbc1::new(Mbc1Type::None).into()),
    0x02 => Ok(CartridgeMbc1::new(Mbc1Type::Ram).into()),
    0x03 => Ok(CartridgeMbc1::new(Mbc1Type::RamBattery).into()),
    _ => Err(YargeError::InvalidMbcType(cart_type))
  }
}
