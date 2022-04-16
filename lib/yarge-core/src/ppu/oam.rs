use std::cmp::Ordering;

#[derive(Clone, Copy, Default)]
pub struct OamFlags {
  pub priority: bool, //BG/Sprite order
  pub flip_y: bool,
  pub flip_x: bool,
  pub palette: bool, //DMG ONLY
}
impl OamFlags {
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
impl Into<u8> for OamFlags {
  fn into(self) -> u8 { self.into_u8() }
}
impl From<u8> for OamFlags {
  fn from(v: u8) -> Self {
    let mut new = Self::default();
    new.from_u8(v);
    return new;
  }
}

#[derive(Clone, Copy, Default)]
pub struct OamObject {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: OamFlags,
  pub id: u8
}
impl OamObject {
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
impl Ord for OamObject {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.x, &self.id).cmp(&(other.x, &other.id))
  }
}
impl PartialOrd for OamObject {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl PartialEq for OamObject {
  fn eq(&self, other: &Self) -> bool {
    (self.x, self.id) == (other.x, other.id)
  }
}
impl Eq for OamObject {}

pub struct OamMemory {
  objects: Box<[OamObject; 40]>,
}
impl OamMemory {
  pub fn new() -> Self {
    let mut objects = Box::new([OamObject::default(); 40]);
    for (i, v) in objects.iter_mut().enumerate() {
      v.id = i as u8;
    }
    Self { objects }
  }
  pub fn get(&self, i: u8) -> OamObject {
    self.objects[i as usize]
  }
  pub fn get_ref(&self, i: u8) -> &OamObject {
    &self.objects[i as usize]
  }
  pub fn get_mut(&mut self, i: u8) -> &mut OamObject {
    &mut self.objects[i as usize]
  }
  pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.objects[(addr >> 2) as usize].set_byte((addr & 3) as u8, value);
  }
  pub fn read_oam(&self, addr: u16) -> u8 {
    self.objects[(addr >> 2) as usize].get_byte((addr & 3) as u8)
  }
}
impl Default for OamMemory {
  fn default() -> Self { Self::new() }
}
