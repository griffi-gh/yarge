use crate::consts::{WIDTH, HEIGHT};
use super::ppu_registers::Lcdc;
use arrayvec::ArrayVec;

#[derive(Clone, Copy, Default)]
pub struct OamFlags {
  pub priority: bool, //BG/Sprite order
  pub flip_y: bool,
  pub flip_x: bool,
  pub palette: bool, //DMG ONLY
}
impl From<OamFlags> for u8 {
  fn from(flags: OamFlags) -> u8 { 
    ((flags.priority as u8) << 7) |
    ((flags.flip_x   as u8) << 6) |
    ((flags.flip_y   as u8) << 5) |
    ((flags.palette  as u8) << 4) 
  }
}
impl From<u8> for OamFlags {
  fn from(value: u8) -> Self {
    Self {
      priority: (value & (1 << 7)) != 0,
      flip_y:   (value & (1 << 6)) != 0,
      flip_x:   (value & (1 << 5)) != 0,
      palette:  (value & (1 << 4)) != 0,
    }
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
      3 => self.flags.into(),
      _ => unreachable!()
    }
  }
  pub fn set_byte(&mut self, byte: u8, value: u8) {
    match byte & 3 {
      0 => { self.y = value; },
      1 => { self.x = value; },
      2 => { self.tile = value; },
      3 => { self.flags = value.into(); },
      _ => unreachable!()
    }
  }
}

pub struct OamMemory {
  pub objects: [OamObject; 40],
}
impl OamMemory {
  pub fn new() -> Self {
    let mut objects = [OamObject::default(); 40];
    for (i, v) in objects.iter_mut().enumerate() {
      v.id = i as u8;
    }
    Self { objects }
  }
  pub fn get_buffer(&self, ly: u8, lcdc: &Lcdc) -> OamBuffer {
    let mut buffer = OamBuffer::new();
    for object in self.objects.iter() {
      let push_cond = {
        (object.x > 0) &&
        ((ly + 16) >= object.y) &&
        ((ly + 16) < (object.y + lcdc.obj_size())) &&
        (object.x <= (WIDTH - 7) as u8)
      };
      if push_cond {
        buffer.push(*object);
        if buffer.len() >= 10 {
          break;
        }
      }
    }
    buffer.sort();
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
  objects: ArrayVec<OamObject, 10>
}
impl OamBuffer {
  pub fn new() -> Self {
    Self {
      objects: ArrayVec::new()
    }
  }
  pub fn push(&mut self, object: OamObject) {
    debug_assert!(self.len() < 10);
    self.objects.push(object);
  }
  pub fn sort(&mut self) {
    self.objects.sort_unstable_by_key(|o| (o.x, o.id));
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
// impl<'a> IntoIterator for &'a OamBuffer {
//   type Item = &'a OamObject;
//   type IntoIter = arrayvec::IntoIter<Self::Item, 10>;
//   fn into_iter(self) -> Self::IntoIter {
//     self.objects.into()
//   }
// }
