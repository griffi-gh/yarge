use std::error::Error;
use std::fmt;
use crate::Res;
use crate::errors::{RomLoadError, InvalidMBCError};

#[allow(unused_variables)]
pub trait Cartridge {
  fn index(&self) -> u8;
  fn name(&self) -> &str;
  fn read(&self, addr: u16) -> u8;
  fn write(&self, addr: u16, value: u8) {}
  fn read_eram(&self, addr: u16) -> u8 { 0xff }
  fn write_eram(&self, addr: u16, value: u8) {}
  fn load(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>>;
}
pub type DynCartridge = Box<(dyn Cartridge + Send)>;

pub struct CartridgeNone {
  index: u8,
  rom: [u8; 0x8000],
}
impl CartridgeNone {
  pub fn new(index: u8) -> Self {
    Self {
      index,
      rom: [0; 0x8000],
    }
  }
}
impl Cartridge for CartridgeNone {
  fn name(&self) -> &str { "MBC0" }
  fn index(&self) -> u8 { self.index }
  fn load(&mut self, rom: &[u8]) -> Res<()> {
    if rom.len() != 0x8000 {
      return Err(
        Box::new(RomLoadError {
          reason: format!(
            "Invalid ROM size: {:#X}.\nPlease note that that MBC cartridges (games larger then 32kb) are not supported yet",
            rom.len()
          )
        })
      );
    }
    for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
      *place = *data;
    }
    Ok(())
  }
  #[inline(always)]
  fn read(&self, addr: u16) -> u8 {
    //bitwise and allows the compiler to optimize away the bounds checks
    //...but I want to keep them on debug buils
    #[cfg(debug_assertions)]
    return self.rom[addr as usize];
    #[cfg(not(debug_assertions))]
    return self.rom[(addr & 0x7FFF) as usize];
  }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct RomHeader {
  pub cart_type: u8,
}
impl fmt::Display for RomHeader {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "MBC Type: {:#04X}", self.cart_type)
  }
}
pub fn parse_header(rom: &[u8]) -> RomHeader {
  RomHeader {
    cart_type: rom[0x147]
  }
}
pub fn get_cartridge(cart_type: u8) -> Res<DynCartridge> {
  match cart_type {
    0x00 => Ok(Box::new(CartridgeNone::new(cart_type))),
    _ => Err(Box::new(InvalidMBCError { mbc: cart_type }))
  }
}
