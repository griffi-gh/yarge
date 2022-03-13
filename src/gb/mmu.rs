pub mod cartridge;

use cartridge::{DynCartridge, get_cartridge};

mod bios;
use bios::BIOS;

pub struct MMU {
  cart: DynCartridge,
  bios_disabled: bool,
}
impl MMU {
  pub fn new() -> Self {
    Self {
      cart: get_cartridge(0),
      bios_disabled: false,
    }
  }
  
  // MAYBE? rename to r16/w16/r8/w8 ?

  #[inline(never)]
  pub fn rb(&self, addr: u16) -> u8 {
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
  #[inline(never)]
  pub fn wb(&mut self, addr: u16, value: u8) {
    match addr {
      0..=0xff => {
        if !self.bios_disabled {
          self.cart.write(addr, value);
        }
      },
      0x100..=0x7fff => self.cart.write(addr, value),
      _ => {}
    }
  }

  #[inline]
  pub fn rw(&self, addr: u16) -> u16 {
    self.rb(addr) as u16 | 
    ((self.rb(addr.wrapping_add(1)) as u16) << 8)
  }
  #[inline]
  pub fn ww(&mut self, addr: u16, value: u16) {
    self.wb(addr, (value & 0xFF) as u8);
    self.wb(addr.wrapping_add(1), (value >> 8) as u8);
  }
}
