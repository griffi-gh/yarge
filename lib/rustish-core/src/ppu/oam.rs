use std::cmp::Ordering;

#[derive(Clone, Copy, Default)]
pub struct OAMFlags {
  pub priority: bool, //BG/Sprite order
  pub flip_y: bool,
  pub flip_x: bool,
  pub palette: bool, //DMG ONLY
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
pub struct OAMObject {
  pub y: u8,
  pub x: u8,
  pub tile: u8,
  pub flags: OAMFlags,
  pub id: u8
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
impl Ord for OAMObject {
  fn cmp(&self, other: &Self) -> Ordering {
    (self.x, &self.id).cmp(&(other.x, &other.id))
  }
}
impl PartialOrd for OAMObject {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl PartialEq for OAMObject {
  fn eq(&self, other: &Self) -> bool {
    (self.x, self.id) == (other.x, other.id)
  }
}
impl Eq for OAMObject {}

#[derive(Clone)]
pub struct OAMMemory {
  pub objects: [OAMObject; 40],
}
impl OAMMemory {
  pub fn new() -> Self {
    let mut objects = [OAMObject::default(); 40];
    for (i, v) in objects.iter_mut().enumerate() {
      v.id = i as u8;
    }
    Self { objects }
  }
  #[allow(dead_code)]
  pub fn get(&self, i: u8) -> &OAMObject {
    &self.objects[i as usize]
  }
  #[allow(dead_code)]
  pub fn get_mut(&mut self, i: u8) -> &mut OAMObject {
    &mut self.objects[i as usize]
  }
  pub fn write_oam(&mut self, addr: u16, value: u8) {
    self.objects[(addr >> 2) as usize].set_byte((addr & 3) as u8, value);
  }
  pub fn read_oam(&self, addr: u16) -> u8 {
    self.objects[(addr >> 2) as usize].get_byte((addr & 3) as u8)
  }
}
impl Default for OAMMemory {
  fn default() -> Self { Self::new() }
}