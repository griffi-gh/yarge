//NOTE: this is just a stub, only disconnected cable is implemented at the moment

use crate::cpu::{Cpu, Interrupt};

pub struct Serial {
  transfer: bool,
  ext_clock: bool,
  data: u8,
}

impl Serial {
  pub fn new() -> Self {
    Self {
      transfer: false,
      ext_clock: false,
      data: 0xff,
    }
  }

  // pub fn shift(&mut self) {
  //   self.data <<= 1;
  //   self.data |= 1;
  // }

  pub fn tick(&mut self, iif: &mut u8) {
    if self.transfer {
      self.transfer = false;
      self.data = 0xff;
      //FIXME: freezes in pokemon red while entering a pokemart
      //Cpu::set_interrupt(iif, Interrupt::Serial);
    }
  }

  pub fn read_sb(&self) -> u8 {
    self.data
  }

  pub fn write_sb(&mut self, value: u8) {
    self.data = value;
  }

  pub fn read_sc(&self) -> u8 {
    (self.transfer as u8) << 7 |
    self.ext_clock as u8 |
    0b0111_1110
  }

  pub fn write_sc(&mut self, value: u8) {
    self.transfer = value & 0x80 != 0;
    self.ext_clock = value & 0x01 != 0;
  }
}

impl Default for Serial {
  fn default() -> Self { Self::new() }
}
