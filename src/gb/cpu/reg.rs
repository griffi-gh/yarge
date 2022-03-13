#[cfg_attr(feature = "no_unsafe", path = "reg/union_safe.rs")]
mod union;

use union::SafeU16Union;

#[derive(Clone, Copy)]
pub struct Registers {
  af: SafeU16Union,
  bc: SafeU16Union,
  de: SafeU16Union,
  hl: SafeU16Union,
  pub pc: u16,
  pub sp: u16,
}
impl Registers {
  pub fn new() -> Self {
      Self {
          af: SafeU16Union::default(),
          bc: SafeU16Union::default(),
          de: SafeU16Union::default(),
          hl: SafeU16Union::default(),
          pc: 0, sp: 0,
      }
  }

  //Inc/Dec
  #[inline]
  pub fn inc_pc(&mut self, by: u16) -> u16 {
    self.pc = self.pc.wrapping_add(by);
    self.pc
  }
  #[inline]
  pub fn dec_pc(&mut self, by: u16) -> u16 {
    self.pc = self.pc.wrapping_sub(by);
    self.pc
  }
  #[inline]
  pub fn inc_sp(&mut self, by: u16) -> u16 {
    self.sp = self.sp.wrapping_add(by);
    self.sp
  }
  #[inline]
  pub fn dec_sp(&mut self, by: u16) -> u16 {
    self.sp = self.sp.wrapping_sub(by);
    self.sp
  }

  // Union registers get
  #[inline]
  pub fn af(&self) -> u16 { self.af.get_union_value() }
  #[inline]
  pub fn bc(&self) -> u16 { self.bc.get_union_value() }
  #[inline]
  pub fn de(&self) -> u16 { self.de.get_union_value() }
  #[inline]
  pub fn hl(&self) -> u16 { self.hl.get_union_value() }

  // Union registers set
  #[inline]
  pub fn set_af(&mut self, value: u16) {
    self.af.set_union_value(value);
  }
  #[inline]
  pub fn set_bc(&mut self, value: u16) {
    self.bc.set_union_value(value);
  }
  #[inline]
  pub fn set_de(&mut self, value: u16) { 
    self.de.set_union_value(value);
  }
  #[inline]
  pub fn set_hl(&mut self, value: u16) {
    self.hl.set_union_value(value);
  }
  
  // 8-bit registers get
  #[inline]
  pub fn a(&self) -> u8 { self.af.get_inner_a() }
  #[inline]
  pub fn f(&self) -> u8 { self.af.get_inner_b() }
  #[inline]
  pub fn b(&self) -> u8 { self.bc.get_inner_a() }
  #[inline]
  pub fn c(&self) -> u8 { self.bc.get_inner_b() }
  #[inline]
  pub fn d(&self) -> u8 { self.de.get_inner_a() }
  #[inline]
  pub fn e(&self) -> u8 { self.de.get_inner_b() }
  #[inline]
  pub fn h(&self) -> u8 { self.hl.get_inner_a() }
  #[inline]
  pub fn l(&self) -> u8 { self.hl.get_inner_b() }
  
  // 8-bit registers set
  #[inline]
  pub fn set_a(&mut self, value: u8) {
    self.af.set_inner_a(value)
  }
  #[inline]
  pub fn set_f(&mut self, value: u8) {
    self.af.set_inner_b(value)
  }
  #[inline]
  pub fn set_b(&mut self, value: u8) {
    self.bc.set_inner_a(value)
  }
  #[inline]
  pub fn set_c(&mut self, value: u8) {
    self.bc.set_inner_b(value)
  }
  #[inline]
  pub fn set_d(&mut self, value: u8) {
    self.de.set_inner_a(value)
  }
  #[inline]
  pub fn set_e(&mut self, value: u8) {
    self.de.set_inner_b(value)
  }
  #[inline]
  pub fn set_h(&mut self, value: u8) {
    self.hl.set_inner_a(value)
  }
  #[inline]
  pub fn set_l(&mut self, value: u8) {
    self.hl.set_inner_b(value)
  }
}
