use enum_dispatch::enum_dispatch;
use crate::{Res, YargeError};

mod helpers;

mod types;
pub use types::*;

mod header;
pub use header::RomHeader;

#[enum_dispatch]
#[allow(unused_variables)]
pub trait CartridgeImpl {
  fn name(&self) -> &'static str;
  
  fn load_rom(&mut self, data: &[u8]) -> Res<()> { Ok(()) }

  fn read_rom(&self, addr: u16) -> u8;
  fn write_rom(&mut self, addr: u16, value: u8) { }

  fn read_eram(&self, addr: u16, blocking: bool) -> u8 { 0xff }
  fn write_eram(&mut self, addr: u16, value: u8, blocking: bool) {}

  fn has_save_data(&self) -> bool { false }
  fn save_data(&self) -> Option<Vec<u8>> { None }
  fn load_data(&mut self, data: Vec<u8>) {}
}

#[non_exhaustive]
#[allow(clippy::enum_variant_names)]
#[enum_dispatch(CartridgeImpl)]
pub enum Cartridge {
  MockCartridge,
  CartridgeNone,
  CartridgeMbc1,
  CartridgeMbc3,
}

pub fn get_cartridge(header: RomHeader) -> Res<Cartridge> {
  let mbc_type = header.mbc_type;
  match mbc_type {
    0x00 => Ok(CartridgeNone::new().into()),
    0x01 => Ok(CartridgeMbc1::new(Mbc1Type::None, &header).into()),
    0x02 => Ok(CartridgeMbc1::new(Mbc1Type::Ram, &header).into()),
    0x03 => Ok(CartridgeMbc1::new(Mbc1Type::RamBattery, &header).into()),
    0x0F => Ok(CartridgeMbc3::new(Mbc3Config {
      timer: true,
      ram: false,
      battery: true,
    }, &header).into()),
    0x10 => Ok(CartridgeMbc3::new(Mbc3Config {
      timer: true,
      ram: true,
      battery: true,
    }, &header).into()),
    0x11 => Ok(CartridgeMbc3::new(Mbc3Config {
      timer: false,
      ram: false,
      battery: false,
    }, &header).into()),
    0x12 => Ok(CartridgeMbc3::new(Mbc3Config {
      timer: false,
      ram: true,
      battery: false,
    }, &header).into()),
    0x13 => Ok(CartridgeMbc3::new(Mbc3Config {
      timer: false,
      ram: true,
      battery: true,
    }, &header).into()),
    _ => Err(YargeError::InvalidMbcType(mbc_type))
  }
}
