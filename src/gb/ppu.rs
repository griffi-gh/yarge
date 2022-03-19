#[derive(Clone, Copy, Default)]
struct OAMFlags {
  pub priority: bool, //BG/Sprite order
  pub flip_y: bool,
  pub flip_x: bool,
  pub palette: bool,
  //GBC other flags
}
impl OAMFlags {
  pub fn into_u8(&self) -> u8 {
    ((self.priority as u8) << 7) |
    ((self.flip_x   as u8) << 6) |
    ((self.flip_y   as u8) << 5) |
    ((self.palette  as u8) << 4) 
  }
  pub fn from_u8(&mut self, v: u8) {
    self.priority = (v & (1 << 7)) != 0;
    self.flip_y   = (v & (1 << 6)) != 0;
    self.flip_x   = (v & (1 << 5)) != 0;
    self.palette  = (v & (1 << 4)) != 0;
  }
}
impl Into<u8> for OAMFlags {
  fn into(self) -> u8 { self.into_u8() }
}
impl From<u8> for OAMFlags {
  fn from(v: u8) -> Self {
    let mut new = Self::default();
    new.from_u8(v);
    return new;
  }
}

#[derive(Clone, Copy, Default)]
struct OAMObject {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: OAMFlags,
}
impl OAMObject {
  pub fn get_byte(&self, byte: u8) -> u8 {
    match byte & 3 {
      0 => self.y,
      1 => self.x,
      2 => self.tile,
      3 => self.flags.into_u8(),
      _ => unreachable!()
    }
  }
  pub fn set_byte(&mut self, byte: u8, value: u8) {
    match byte & 3 {
      0 => { self.y = value; },
      1 => { self.x = value; },
      2 => { self.tile = value; },
      3 => { self.flags.from_u8(value); },
      _ => unreachable!()
    }
  }
}

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
  oam: [OAMObject; 40],
}
impl PPU {
  pub fn new() -> Self {
    Self {
      vram: [0; 0x2000],
      oam: [OAMObject::default(); 40],
    }
  }
  pub fn write_oam(&mut self, addr: u16, value: u8) {
    //TEST
    self.oam[(addr >> 2) as usize].set_byte((addr & 3) as u8, value);
  }
  pub fn read_oam(&self, addr: u16) -> u8 {
    self.oam[(addr >> 2) as usize].get_byte((addr & 3) as u8)
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