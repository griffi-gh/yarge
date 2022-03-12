pub mod cartridge;

use cartridge::{Cartridge, CartridgeNone};

mod bios;
use bios::BIOS;

pub struct MMU {
  cart: Box<(dyn Cartridge + Send)>,
  bios_disabled: bool,
}
impl MMU {
  pub fn new() -> Self {
    Self {
      cart: Box::new(CartridgeNone::new()),
      bios_disabled: false,
    }
  }
  pub fn read(&self, addr: u16) -> u8 {
    match addr {
      0..=0xff => {
        if self.bios_disabled {
          self.cart.read(addr)
        } else {
          BIOS[addr as usize]
        }
      },
      0x100..=0x7fff => self.cart.read(addr),
      _ => 0xff
    }
  }
  pub fn write(&mut self, addr: u16, value: u8) {
    match addr {
      0..=0xff => {
        if !self.bios_disabled {
          self.cart.write(addr, value);
        }
      },
      0x100..=0x7fff => self.cart.write(addr, value),
      _=>{}
    }
  }
}