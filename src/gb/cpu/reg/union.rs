#![forbid(unsafe_code)]

#[derive(Clone, Copy)]
pub struct U16Union {
  value: u16
}

//MAYBE store two u8?

impl U16Union {
  pub fn new(value: u16) -> Self { Self { value } }

  #[inline(always)]
  pub fn set(&mut self, value: u16) {
    self.value = value;
  }
  #[inline(always)]
  pub fn get(&self) -> u16 {
    self.value
  }

  #[inline]
  pub fn set_a(&mut self, value: u8) {
    self.value = (self.value & 0x00FF) | ((value as u16) << 8);
  }
  #[inline]
  pub fn set_b(&mut self, value: u8) {
    self.value = (self.value & 0xFF00) | (value as u16);
  }

  #[inline]
  pub fn get_a(&self) -> u8 {
    (self.value >> 8) as u8
  }
  #[inline]
  pub fn get_b(&self) -> u8 {
    (self.value & 0xFF) as u8
  }
}
impl Default for U16Union {
  fn default() -> Self { Self::new(0) }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn union_create_get() {
    let mut a = U16Union::new(0x1234);
    let mut b = U16Union::default();
    assert_eq!(a.get(), 0x1234);
    assert_eq!(b.get(), 0);
  }

  #[test]
  fn union_set_get_a_b() {
    let mut a = U16Union::new(0x1234);
    assert_eq!(a.get_a(), 0x12);
    assert_eq!(a.get_b(), 0x34);
    a.set_a(0x56);
    a.set_b(0x78);
    assert_eq!(a.get_a(), 0x56);
    assert_eq!(a.get_b(), 0x78);
    assert_eq!(a.get(), 0x5678);
  }

  #[test]
  fn union_get_immut() {
    let a = U16Union::new(0x5678);
    assert_eq!(a.get_a(), 0x56);
    assert_eq!(a.get_b(), 0x78);
    assert_eq!(a.get(), 0x5678);
  }
}