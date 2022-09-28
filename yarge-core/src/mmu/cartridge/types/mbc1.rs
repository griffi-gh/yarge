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

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Type {
  None, Ram, RamBattery
}

pub struct CartridgeMbc1 {
  mbc1_type: Type,
  rom: Vec<u8>,
  eram: Option<Vec<u8>>,
  rom_mask: u8,
  ram_mask: u8,
  rom_bank: u8,
  ram_bank: u8,
  ram_enable: bool,
  mode: bool,
}
impl CartridgeMbc1 {
  pub fn new(mbc1_type: Type, header: &RomHeader) -> Self {
    Self {
      mbc1_type,
      rom: Vec::with_capacity(0x8000),
      eram: (mbc1_type != Type::None).then(|| vec![0; header.ram_size.max(8192)]),
      rom_mask: rom_bank_mask(header),
      ram_mask: eram_bank_mask(header),
      rom_bank: 1,
      ram_bank: 0,
      ram_enable: false,
      mode: false,
    }
  }
}
impl CartridgeImpl for CartridgeMbc1 {
  fn name(&self) -> &'static str { "MBC1" }

  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    load_rom_vec(&mut self.rom, rom)
  }

  fn read_rom(&self, addr: u16) -> u8 {
    if addr < 0x4000 {
      return self.rom[addr as usize];
    }
    let mut bank = self.rom_bank;
    if self.mode {
      bank += self.ram_bank << 5;
    }
    bank &= self.rom_mask;
    self.rom[rom_addr(addr, bank)]
  }
  fn write_rom(&mut self, addr: u16, value: u8) {
    match addr {
      0x0000..=0x1FFF => {
        self.ram_enable = (value & 0xF) == 0xA;
      },
      0x2000..=0x3FFF => {
        self.rom_bank = (value & 0x1F).max(1);
      },
      0x4000..=0x5FFF => {
        self.ram_bank = value & 0b11;
      },
      0x6000..=u16::MAX => {
        self.mode = (value & 1) != 0;
      }
    }
  }

  fn read_eram(&self, addr: u16, blocking: bool) -> u8 {
    if self.mbc1_type == Type::None { return 0xFF }
    if blocking && !self.ram_enable { return 0xFF }
    let bank = if self.mode {
      self.ram_bank & self.ram_mask
    } else {
      0x00
    };
    self.eram.as_ref().unwrap()[eram_addr(addr, bank)]
  }
  fn write_eram(&mut self, addr: u16, value: u8, blocking: bool) {
    if self.mbc1_type == Type::None { return }
    if blocking && !self.ram_enable { return }
    let bank = if self.mode {
      self.ram_bank & self.ram_mask
    } else {
      0x00
    };
    self.eram.as_mut().unwrap()[eram_addr(addr, bank)] = value;
  }

  fn save_data(&self) -> Option<Vec<u8>> {
    match self.mbc1_type {
      Type::RamBattery => Some(self.eram.as_ref().unwrap().clone()),
      _ => None
    }
  }
  fn load_data(&mut self, data: Vec<u8>) {
    //TODO add checks
    self.eram = Some(data);
  }
}
