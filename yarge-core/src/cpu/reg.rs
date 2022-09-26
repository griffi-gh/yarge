mod union;
use union::U16Union;

//TODO Clean up this fucking mess

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
  pub fn inc_pc(&mut self, by: u16) -> u16 {
    self.pc = self.pc.wrapping_add(by);
    self.pc
  }
  pub fn dec_pc(&mut self, by: u16) -> u16 {
    self.pc = self.pc.wrapping_sub(by);
    self.pc
  }
  pub fn inc_sp(&mut self, by: u16) -> u16 {
    self.sp = self.sp.wrapping_add(by);
    self.sp
  }
  pub fn dec_sp(&mut self, by: u16) -> u16 {
    self.sp = self.sp.wrapping_sub(by);
    self.sp
  }

  // Union registers get
  pub fn af(&self) -> u16 { self.af.get() & 0xFFF0 }
  pub fn bc(&self) -> u16 { self.bc.get() }
  pub fn de(&self) -> u16 { self.de.get() }
  pub fn hl(&self) -> u16 { self.hl.get() }
  
  // Union registers set
  pub fn set_af(&mut self, value: u16) { self.af.set(value); }
  pub fn set_bc(&mut self, value: u16) { self.bc.set(value); }
  pub fn set_de(&mut self, value: u16) { self.de.set(value); }
  pub fn set_hl(&mut self, value: u16) { self.hl.set(value); }

  // 8-bit registers get
  pub fn a(&self) -> u8 { self.af.get_a() }
  pub fn f(&self) -> u8 { self.af.get_b() & 0xF0 }
  pub fn b(&self) -> u8 { self.bc.get_a() }
  pub fn c(&self) -> u8 { self.bc.get_b() }
  pub fn d(&self) -> u8 { self.de.get_a() }
  pub fn e(&self) -> u8 { self.de.get_b() }
  pub fn h(&self) -> u8 { self.hl.get_a() }
  pub fn l(&self) -> u8 { self.hl.get_b() }
  
  // 8-bit registers set
  pub fn set_a(&mut self, value: u8) { self.af.set_a(value); }
  pub fn set_f(&mut self, value: u8) { self.af.set_b(value); }
  pub fn set_b(&mut self, value: u8) { self.bc.set_a(value); }
  pub fn set_c(&mut self, value: u8) { self.bc.set_b(value); }
  pub fn set_d(&mut self, value: u8) { self.de.set_a(value); }
  pub fn set_e(&mut self, value: u8) { self.de.set_b(value); }
  pub fn set_h(&mut self, value: u8) { self.hl.set_a(value); }
  pub fn set_l(&mut self, value: u8) { self.hl.set_b(value); }

  // 16-bit reg setters/getters, for compatability.
  pub fn set_sp(&mut self, value: u16) { self.sp = value; }
  pub fn sp(&self) -> u16 { self.sp }
  pub fn set_pc(&mut self, value: u16) { self.pc = value; }
  pub fn pc(&self) -> u16 { self.pc }

  // Flag register
  pub fn f_z(&self) -> bool { (self.f() & 0x80) != 0 }
  pub fn f_n(&self) -> bool { (self.f() & 0x40) != 0 }
  pub fn f_h(&self) -> bool { (self.f() & 0x20) != 0 }
  pub fn f_c(&self) -> bool { (self.f() & 0x10) != 0 }

  // Flag register inverse, to simplify conditionals
  pub fn f_nz(&self) -> bool { !self.f_z() }
  pub fn f_nc(&self) -> bool { !self.f_c() }

  // Flag reg set
  pub fn set_f_z(&mut self, value: bool) {
    self.set_f((self.f() & 0b01111111) | (value as u8) << 7);
  }
  pub fn set_f_n(&mut self, value: bool) {
    self.set_f((self.f() & 0b10111111) | (value as u8) << 6);
  }
  pub fn set_f_h(&mut self, value: bool) {
    self.set_f((self.f() & 0b11011111) | (value as u8) << 5);
  }
  pub fn set_f_c(&mut self, value: bool) {
    self.set_f((self.f() & 0b11101111) | (value as u8) << 4);
  }

  pub fn set_f_znhc(&mut self, z: bool, n: bool, h: bool, c: bool) {
    self.set_f(((z as u8) << 7) | ((n as u8) << 6) | ((h as u8) << 5) | ((c as u8) << 4));
  }
}
