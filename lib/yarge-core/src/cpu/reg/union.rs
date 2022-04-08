#[derive(Clone, Copy)]
pub struct U16Union {
  value: u16
}

impl U16Union {
  #[inline] pub fn new(value: u16) -> Self {
    Self { value }
  }

  #[inline] pub fn set(&mut self, value: u16) {
    self.value = value;
  }
  #[inline] pub fn get(&self) -> u16 {
    self.value
  }

  #[inline] pub fn set_a(&mut self, value: u8) {
    self.value = (self.value & 0x00FF) | ((value as u16) << 8);
  }
  #[inline] pub fn set_b(&mut self, value: u8) {
    self.value = (self.value & 0xFF00) | (value as u16);
  }

  #[inline] pub fn get_a(&self) -> u8 {
    (self.value >> 8) as u8
  }
  #[inline] pub fn get_b(&self) -> u8 {
    (self.value & 0xFF) as u8
  }
}
impl Into<u16> for U16Union {
  fn into(self) -> u16 { self.get() }
}
impl From<u16> for U16Union {
  fn from(val: u16) -> Self { Self::new(val) }
}
impl Default for U16Union {
  fn default() -> Self { Self::new(0) }
}
