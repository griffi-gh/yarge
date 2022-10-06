//TODO Handle RTC/Timer

use crate::Res;
use super::{
  helpers::{
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
pub struct Configuration {
  pub timer: bool,
  pub ram: bool,
  pub battery: bool,
}

pub struct CartridgeMbc3 {
  config: Configuration,
  rom: Vec<u8>,
  eram: Option<Vec<u8>>,
  rom_mask: u8,
  ram_mask: u8,
  rom_bank: u8,
  ram_bank: u8,
  ram_enable: bool
}
impl CartridgeMbc3 {
  pub fn new(config: Configuration, header: &RomHeader) -> Self {
    Self {
      config,
      rom: Vec::with_capacity(0x8000),
      eram: config.ram.then(|| vec![0; header.ram_size.max(8192)]),
      rom_mask: rom_bank_mask(header),
      ram_mask: eram_bank_mask(header),
      rom_bank: 1,
      ram_bank: 0,
      ram_enable: false,
    }
  }
}
impl CartridgeImpl for CartridgeMbc3 {
  fn name(&self) -> &'static str { "MBC3" }

  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    load_rom_vec(&mut self.rom, rom)
  }

  fn read_rom(&self, addr: u16) -> u8 { 
    match addr {
      0x0000..=0x3FFF => self.rom[addr as usize],
      0x4000..=0xFFFF => self.rom[rom_addr(addr, self.rom_bank)],
    }
  }
  fn write_rom(&mut self, addr: u16, value: u8) {
    match addr {
      0x0000..=0x1FFF => {
        self.ram_enable = (value & 0xF) == 0xA;
      },
      0x2000..=0x3FFF => {
        self.rom_bank = (value & self.rom_mask).max(1);
      },
      0x4000..=0x5FFF => {
        if !(0x08..=0x0C).contains(&value) {
          self.ram_bank = value & 0b11;
        }
      },
      0x6000..=0xFFFF => {}
    }
  }

  fn read_eram(&self, addr: u16, blocking: bool) -> u8 {
    if !self.config.ram { return 0xFF }
    if blocking && !self.ram_enable { return 0xFF }
    self.eram.as_ref().unwrap()[eram_addr(addr, self.ram_bank)]
  }
  fn write_eram(&mut self, addr: u16, value: u8, blocking: bool) {
    if !self.config.ram { return }
    if blocking && !self.ram_enable { return }
    self.eram.as_mut().unwrap()[eram_addr(addr, self.ram_bank)] = value;
  }

  fn save_data(&self) -> Option<Vec<u8>> {
    (self.config.battery && self.config.ram).then(|| {
      self.eram.as_ref().unwrap().clone()
    })
  }
  fn load_data(&mut self, data: Vec<u8>) {
    self.eram = Some(data);
  }
}
