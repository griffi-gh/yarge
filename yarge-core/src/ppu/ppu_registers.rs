#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PpuMode {
  HBlank     = 0,
  VBlank     = 1,
  OamSearch  = 2,
  PxTransfer = 3,
}

impl Default for PpuMode {
  fn default() -> Self { Self::OamSearch }
}

#[derive(Default, Clone, Copy)]
pub struct Lcdc {
  pub enable_bg: bool,
  pub enable_obj: bool,      
  pub obj_size: bool,         // 0: 8x8, 1: 8x16
  pub bg_tilemap_addr: bool,  // 0: 0x9800-0x9BFF, 1: 0x9C00-0x9FFF
  pub tiledata_addr: bool,    // BG/WIN tile data addr 0: 0x8800-0x97FF, 1: 0x8000-0x87FF
  pub enable_win: bool,
  pub win_tilemap_addr: bool, // 0: 0x9800-0x9BFF, 1: 0x9C00-0x9FFF
  pub enable_display: bool,
}
impl Lcdc {
  pub fn from_u8(value: u8) -> Self {
    let mut new = Self::default();
    new.set_from_u8(value);
    new
  }
  pub fn set_from_u8(&mut self, value: u8) {
    self.enable_bg        = (value & 0x01) != 0;
    self.enable_obj       = (value & 0x02) != 0;
    self.obj_size         = (value & 0x04) != 0;
    self.bg_tilemap_addr  = (value & 0x08) != 0;
    self.tiledata_addr    = (value & 0x10) != 0;
    self.enable_win       = (value & 0x20) != 0;
    self.win_tilemap_addr = (value & 0x40) != 0;
    self.enable_display   = (value & 0x80) != 0;
  }
  pub fn into_u8(self) -> u8 {
    (self.enable_bg         as u8)       | 
    ((self.enable_obj       as u8) << 1) |
    ((self.obj_size         as u8) << 2) |
    ((self.bg_tilemap_addr  as u8) << 3) |
    ((self.tiledata_addr    as u8) << 4) |
    ((self.enable_win       as u8) << 5) |
    ((self.win_tilemap_addr as u8) << 6) |
    ((self.enable_display   as u8) << 7)
  }
  pub fn bg_tilemap_addr(&self) -> u16 {
    if self.bg_tilemap_addr { 0x9C00 } else { 0x9800 }
  }
  pub fn win_tilemap_addr(&self) -> u16 {
    //Not sure if it's correct?
    if self.win_tilemap_addr { 0x9C00 } else { 0x9800 }
  }
  pub fn bg_tiledata_addr(&self) -> u16 {
    if self.tiledata_addr { 0x8800 } else { 0x8000 }
  }
  pub fn obj_size(&self) -> u8 {
    if self.obj_size { 16 } else { 8 }
  }
  pub fn transform_tile_index(&self, index: u8) -> u16 {
    if self.tiledata_addr {
      index as u16
    } else { 
      ((index as i8) as u16).wrapping_add(0x100)
    }
  }
}
impl From<Lcdc> for u8 {
  fn from(lcdc: Lcdc) -> u8 {
    lcdc.into_u8()
  }
}
impl From<u8> for Lcdc {
  fn from(v: u8) -> Self {
    Self::from_u8(v)
  }
}

#[derive(Default, Clone, Copy)]
pub struct StatInterrupts {
  pub lyc_eq: bool,
  pub mode_2: bool,
  pub mode_1: bool,
  pub mode_0: bool,
}
impl StatInterrupts {
  pub fn from_u8(value: u8) -> Self {
    let mut new = Self::default();
    new.set_from_u8(value);
    new
  }
  pub fn set_from_u8(&mut self, value: u8) {
    self.mode_0 = (value & 0x01) != 0;
    self.mode_1 = (value & 0x02) != 0;
    self.mode_2 = (value & 0x04) != 0;
    self.lyc_eq = (value & 0x08) != 0;
  }
  pub fn into_u8(self) -> u8 {
    (self.mode_0 as u8)        |
    ((self.mode_1 as u8) << 1) |
    ((self.mode_2 as u8) << 2) |
    ((self.lyc_eq as u8) << 3) 
  }
}
impl From<StatInterrupts> for u8 {
  fn from(intr: StatInterrupts) -> u8 {
    intr.into_u8()
  }
}
impl From<u8> for StatInterrupts {
  fn from(v: u8) -> Self {
    Self::from_u8(v)
  }
}
