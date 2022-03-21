pub mod oam;
use oam::{OAM, OAMObject};

#[derive(Clone, Copy)]
pub enum PPUMode {
  HBlank = 0,
  VBlank = 1,
  OAM    = 2,
  VRAM   = 3,
}
impl PPUMode {
  pub fn from_u8(val: u8) -> Self {
    #[cfg(not(debug_assertions))]
    let mut val = val;
    #[cfg(not(debug_assertions))] {
      val &= 3;
    }
    match val {
      0 => Self::HBlank,
      1 => Self::VBlank,
      2 => Self::OAM,
      3 => Self::VRAM,
      #[cfg(not(debug_assertions))]
      _ => unreachable!(),
      #[cfg(debug_assertions)]
      _ => panic!("Invalid mode"),
    }
  }
  pub fn into_u8(&self) -> u8 {
    *self as u8
  }
}
impl From<u8> for PPUMode {
  fn from(val: u8) -> Self { Self::from_u8(val) }
}
impl Into<u8> for PPUMode {
  fn into(self) -> u8 { self.into_u8() }
}
impl Default for PPUMode {
  fn default() -> Self { Self::HBlank }
}

pub struct PPU {
  vram: [u8; 0x2000],
  oam: OAM,
  pub ly: u8,
}
impl PPU {
  pub fn new() -> Self {
    Self {
      vram: [0; 0x2000],
      oam: OAM::default(),
      ly: 0,
    }
  }
  pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.oam.write_mem(addr as usize, value);
  }
  pub fn read_oam(&self, addr: u16) -> u8 {
    self.oam.read_mem(addr as usize)
  }
  pub fn write_vram(&mut self, addr: u16, value: u8) {
    self.vram[(addr & 0x1FFF) as usize] = value;
  }
  pub fn read_vram(&self, addr: u16) -> u8 {
    self.vram[(addr & 0x1FFF) as usize]
  }
  pub fn tick(&self, _t: u32) {
    //TODO
  }
}