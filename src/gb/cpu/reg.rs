mod union;

use union::U16Union;

#[derive(Clone, Copy)]
pub struct Registers {
  af: U16Union,
  bc: U16Union,
  de: U16Union,
  hl: U16Union,
  pub pc: u16,
  pub sp: u16,
}
impl Registers {
  pub fn new() -> Self {
      Self {
          af: U16Union::default(),
          bc: U16Union::default(),
          de: U16Union::default(),
          hl: U16Union::default(),
          pc: 0, sp: 0,
      }
  }

  //Inc/Dec
  #[inline(always)]
  pub fn inc_pc(&mut self, by: u16) -> u16 {
    self.pc = self.pc.wrapping_add(by);
    self.pc
  }
  #[inline(always)]
  pub fn dec_pc(&mut self, by: u16) -> u16 {
    self.pc = self.pc.wrapping_sub(by);
    self.pc
  }
  #[inline(always)]
  pub fn inc_sp(&mut self, by: u16) -> u16 {
    self.sp = self.sp.wrapping_add(by);
    self.sp
  }
  #[inline(always)]
  pub fn dec_sp(&mut self, by: u16) -> u16 {
    self.sp = self.sp.wrapping_sub(by);
    self.sp
  }

  // Union registers get
  #[inline(always)] pub fn af(&self) -> u16 { self.af.get() }
  #[inline(always)] pub fn bc(&self) -> u16 { self.bc.get() }
  #[inline(always)] pub fn de(&self) -> u16 { self.de.get() }
  #[inline(always)] pub fn hl(&self) -> u16 { self.hl.get() }
  
  // Union registers set
  #[inline(always)] pub fn set_af(&mut self, v: u16) { self.af.set(v); }
  #[inline(always)] pub fn set_bc(&mut self, v: u16) { self.bc.set(v); }
  #[inline(always)] pub fn set_de(&mut self, v: u16) { self.de.set(v); }
  #[inline(always)] pub fn set_hl(&mut self, v: u16) { self.hl.set(v); }

  // 8-bit registers get
  #[inline] pub fn a(&self) -> u8 { self.af.get_a() }
  #[inline] pub fn f(&self) -> u8 { self.af.get_b() }
  #[inline] pub fn b(&self) -> u8 { self.bc.get_a() }
  #[inline] pub fn c(&self) -> u8 { self.bc.get_b() }
  #[inline] pub fn d(&self) -> u8 { self.de.get_a() }
  #[inline] pub fn e(&self) -> u8 { self.de.get_b() }
  #[inline] pub fn h(&self) -> u8 { self.hl.get_a() }
  #[inline] pub fn l(&self) -> u8 { self.hl.get_b() }
  
  // 8-bit registers set
  #[inline] pub fn set_a(&mut self, val: u8) { self.af.set_a(val) }
  #[inline] pub fn set_f(&mut self, val: u8) { self.af.set_b(val) }
  #[inline] pub fn set_b(&mut self, val: u8) { self.bc.set_a(val) }
  #[inline] pub fn set_c(&mut self, val: u8) { self.bc.set_b(val) }
  #[inline] pub fn set_d(&mut self, val: u8) { self.de.set_a(val) }
  #[inline] pub fn set_e(&mut self, val: u8) { self.de.set_b(val) }
  #[inline] pub fn set_h(&mut self, val: u8) { self.hl.set_a(val) }
  #[inline] pub fn set_l(&mut self, val: u8) { self.hl.set_b(val) }

  // 16-bit reg ops, for compatability.
  #[inline(always)] pub fn set_sp(&mut self, val: u16) { self.sp = val; }
  #[inline(always)] pub fn set_pc(&mut self, val: u16) { self.pc = val; }
  #[inline(always)] pub fn sp(&self) -> u16 { self.sp }
  #[inline(always)] pub fn pc(&self) -> u16 { self.pc }

  // TODO Flag register
}
