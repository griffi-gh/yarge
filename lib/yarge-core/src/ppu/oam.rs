use std::{cmp::Ordering, ops::Index};

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
  pub fn from_u8(&mut self, value: u8) {
    self.priority = (value & (1 << 7)) != 0;
    self.flip_y   = (value & (1 << 6)) != 0;
    self.flip_x   = (value & (1 << 5)) != 0;
    self.palette  = (value & (1 << 4)) != 0;
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
  pub objects: Box<[OamObject; 40]>,
}
impl OamMemory {
  pub fn new() -> Self {
    let mut objects = Box::new([OamObject::default(); 40]);
    for (i, v) in objects.iter_mut().enumerate() {
      v.id = i as u8;
    }
    Self { objects }
  }
  pub fn get_buffer() -> OamBuffer {
    let mut buffer = OamBuffer::new();
    //buffer.objects.push();
    buffer
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

pub struct OamBuffer {
  pub objects: Vec<OamObject>,
}
impl OamBuffer {
  pub fn new() -> Self {
    Self {
      objects: Vec::with_capacity(10)
    }
  }
  
  pub fn push(&mut self, obj: OamObject) {
    #[cfg(debug_assertions)]
    assert!(self.len() < 10);
    self.objects.push(obj);
  }
  pub fn sort(&mut self) {
    self.objects.sort_by(|a, b| {
      a.x.partial_cmp(&b.x).unwrap()
    });
  }

  pub fn len(&self) -> usize {
    self.objects.len()
  }
  pub fn get(&self, index: usize) -> Option<&OamObject> {
    self.objects.get(index)
  }
}
impl Default for OamBuffer {
  fn default() -> Self { Self::new() }
}
