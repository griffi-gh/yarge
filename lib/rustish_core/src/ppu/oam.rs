#[derive(Clone, Copy, Default)]
pub struct OAMFlags {
  pub priority: bool, //BG/Sprite order
  pub flip_y: bool,
  pub flip_x: bool,
  pub palette: bool,
  //GBC other flags
}
impl OAMFlags {
  pub fn new() -> Self { Self::default() }
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
pub struct OAMObject {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: OAMFlags,
}
impl OAMObject {
  pub fn new() -> Self { Self::default() }
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

//todo cache oam?

#[derive(Clone, Copy)]
pub struct OAM {
  objects: [OAMObject; 40],
}
impl Default for OAM {
  fn default() -> Self { Self::new() }
}
impl OAM {
  pub fn new() -> Self {
    Self { objects: [OAMObject::default(); 40] }
  }
  #[inline]
  pub fn get(&mut self, idx: usize) -> &mut OAMObject {
    &mut (self.objects[idx])
  }
  pub fn write_mem(&mut self, addr: usize, value: u8) {
    self.objects[addr >> 2].set_byte((addr & 3) as u8, value);
  }
  pub fn read_mem(&self, addr: usize) -> u8 {
    self.objects[addr >> 2].get_byte((addr & 3) as u8)
  }
}