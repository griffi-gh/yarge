#![forbid(unsafe_code)]

#[derive(Clone, Copy)]
pub struct U16Union {
  value: u16
}

impl U16Union {
  pub fn new(value: u16) -> Self { Self { value } }

  //MAYBE rename these functions
  #[inline]
  pub fn set_union_value(&mut self, value: u16) {
    self.value = value;
  }
  #[inline]
  pub fn get_union_value(&self) -> u16 {
    self.value
  }

  #[inline]
  pub fn set_inner_a(&mut self, value: u8) {
    self.value = (self.value & 0x00FF) | ((value as u16) << 8);
  }
  #[inline]
  pub fn set_inner_b(&mut self, value: u8) {
    self.value = (self.value & 0xFF00) | (value as u16);
  }

  #[inline]
  pub fn get_inner_a(&self) -> u8 {
    (self.value >> 8) as u8
  }
  #[inline]
  pub fn get_inner_b(&self) -> u8 {
    (self.value & 0xFF) as u8
  }
}
impl Default for U16Union {
  fn default() -> Self { Self::new(0) }
}