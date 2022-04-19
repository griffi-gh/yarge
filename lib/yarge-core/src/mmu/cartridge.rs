use std::fmt;
use arrayvec::ArrayString;
use enum_dispatch::enum_dispatch;
use crate::{Res, YargeError, consts::DEFAULT_HEADER};

#[allow(unused_variables)]
#[enum_dispatch]
pub trait CartridgeImpl {
  fn name(&self) -> &str;
  fn load_rom(&mut self, data: &[u8]) -> Res<()>;
  fn read_rom(&self, addr: u16) -> u8;
  fn write_rom(&self, addr: u16, value: u8) {}
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

pub struct MockCartridge;
impl CartridgeImpl for MockCartridge {
  fn name(&self) -> &str { "NONE" }
  fn load_rom(&mut self, _: &[u8]) -> Res<()> { Ok(()) }
  fn read_rom(&self, addr: u16) -> u8 {
    if (0x100..(0x100+80)).contains(&addr) {
      DEFAULT_HEADER[(addr - 0x100) as usize]
    } else {
      0x00
    }
  }
}

fn verify_rom_addr(addr: u16) {
  assert!((0..=0x7FFF).contains(&addr), "Out of bounds read");
}
fn verify_eram_addr(addr: u16) {
  assert!((0xA000..=0xBFFF).contains(&addr), "Out of bounds read");
}
macro_rules! verify_rom_addr_if_debug {
  ($addr: expr) => {
    #[cfg(debug_assertions)] {
      verify_rom_addr($addr);
    }
  };
}
macro_rules! verify_eram_addr_if_debug {
  ($addr: expr) => {
    #[cfg(debug_assertions)] {
      verify_eram_addr($addr);
    }
  };
}

pub struct CartridgeNone {
  rom: Box<[u8; 0x8000]>,
}
impl CartridgeNone {
  pub fn new() -> Self {
    Self {
      rom: Box::new([0xFF; 0x8000]),
    }
  }
}
impl CartridgeImpl for CartridgeNone {
  fn name(&self) -> &str { "MBC0" }
  fn read_rom(&self, addr: u16) -> u8 {
    verify_rom_addr_if_debug!(addr);
    return self.rom[(addr & 0x7FFF) as usize];
  }
  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    if rom.len() != 0x8000 {
      return Err(
        YargeError::InvalidRomSize(rom.len())
      );
    }
    for (place, data) in self.rom.iter_mut().zip(rom.iter()) {
      *place = *data;
    }
    Ok(())
  }
}

#[repr(u8)]
enum Mbc1Type {
  None, Ram, RamBattery
}
pub struct CartridgeMbc1 {
  rom: Vec<u8>,
  mbc1_type: Mbc1Type
}
impl CartridgeMbc1 {
  fn new(mbc1_type: Mbc1Type) -> Self {
    Self {
      rom: Vec::with_capacity(0x8000),
      mbc1_type
    }
  }
}
impl CartridgeImpl for CartridgeMbc1 {
  fn name(&self) -> &str { "MBC1" }
  fn load_rom(&mut self, rom: &[u8]) -> Res<()> {
    self.rom.clear();
    self.rom.extend_from_slice(rom);
    self.rom.shrink_to_fit();
    Ok(())
  }
  fn read_rom(&self, addr: u16) -> u8 {
    verify_rom_addr_if_debug!(addr);
    todo!()
  }
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

#[derive(Clone, Copy, Default, Debug)]
pub struct RomHeader {
  pub name: ArrayString<16>,
  pub mbc_type: u8,
}
impl RomHeader {
  pub fn parse(rom: &[u8]) -> Self {
    Self {
      mbc_type: rom[0x147],
      name: {
        let mut str = ArrayString::<16>::new();
        for addr in 0x134..=0x143_usize {
          let byte = rom[addr];
          if byte == 0 {
            break;
          } else {
            str.push(char::from_u32(byte as u32).unwrap());
          }
        }
        str
      }
    }
  }
}
impl fmt::Display for RomHeader {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mbc_type = self.mbc_type;
    let name = &self.name[..];
    write!(formatter, "Name: {name}\nMBC Type: {mbc_type:#04X}")
  }
}
