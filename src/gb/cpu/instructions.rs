pub use paste::paste;

macro_rules! ld_rr_u16 {
  ($self: expr, $reg: ident) => { 
    let val = $self.fetch_word();
    paste! { 
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
}
pub(crate) use ld_rr_u16;

macro_rules! ld_mrr_a {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.reg.[<$reg:lower>]();
    } 
    $self.wb(v, $self.reg.a());
  };
}
pub(crate) use ld_mrr_a;

macro_rules! ld_mhli_a {
  ($self: expr, $inc: ident) => {
    let v = $self.reg.hl();
    paste! {
      $self.reg.set_hl($self.reg.hl().[<wrapping_ $inc:lower>](1));
    }
    $self.wb(v, $self.reg.a());
  };
}
pub(crate) use ld_mhli_a;

macro_rules! incdec_rr {
  ($self: expr, $reg: ident, $inc: ident) => {
    paste! {
      $self.reg.[<set_ $reg:lower>](
        $self.reg.[<$reg:lower>]().[<wrapping_ $inc:lower>](1)
      );
    }
    $self.internal(4);
  };
}
pub(crate) use incdec_rr;

macro_rules! ld_r_r {
  ($self: expr, $a: ident, $b: ident) => {
    paste! {
      $self.reg.[<set_ $a:lower>](
        $self.reg.[<$b:lower>]()
      );
    }
  };
}
pub(crate) use ld_r_r;


macro_rules! cpu_instructions {
  ($self: expr, $op: expr) => {
    match($op) {
      0x00 => { },                              //NOP
      0x01 => { ld_rr_u16!($self, BC); },       //LD BC,u16
      0x02 => {  ld_mrr_a!($self, BC); },       //LD (BC),A
      0x03 => { incdec_rr!($self, BC, add); }   //INC BC
      0x0B => { incdec_rr!($self, BC, sub); }   //DEC BC
      0x11 => { ld_rr_u16!($self, DE); },       //LD DE,u16
      0x12 => {  ld_mrr_a!($self, DE); },       //LD (DE),A
      0x13 => { incdec_rr!($self, DE, add); },  //INC DE
      0x1B => { incdec_rr!($self, DE, sub); }   //DEC DE
      0x21 => { ld_rr_u16!($self, HL); },       //LD HL,u16
      0x22 => { ld_mhli_a!($self, add); },      //LD (HL+),A
      0x23 => { incdec_rr!($self, HL, add); },  //INC HL
      0x2B => { incdec_rr!($self, HL, sub); }   //DEC HL
      0x31 => { ld_rr_u16!($self, SP); },       //LD SP,u16
      0x32 => { ld_mhli_a!($self, sub); },      //LD (HL-),A
      0x33 => { incdec_rr!($self, SP, add); },  //INC SP
      0x3B => { incdec_rr!($self, SP, sub); },  //DEC SP

      0x40 => { /*TODO Breakpoint */ }          //LD B,B
      0x41 => { ld_r_r!($self, B, C); }         //LD B,C
      0x42 => { ld_r_r!($self, B, D); }         //LD B,D
      0x43 => { ld_r_r!($self, B, E); }         //LD B,E
      0x44 => { ld_r_r!($self, B, H); }         //LD B,H
      0x45 => { ld_r_r!($self, B, L); }         //LD B,L
      0x47 => { ld_r_r!($self, B, A); }         //LD B,A

      0x48 => { ld_r_r!($self, C, C); }         //LD C,B
      0x49 => { /*IS A NO-OP*/ }                //LD C,C
      0x4A => { ld_r_r!($self, C, D); }         //LD C,D
      0x4B => { ld_r_r!($self, C, E); }         //LD C,E
      0x4C => { ld_r_r!($self, C, H); }         //LD C,H
      0x4D => { ld_r_r!($self, C, L); }         //LD C,L
      0x4F => { ld_r_r!($self, C, A); }         //LD C,A

      _ => panic!("Invalid instruction")
    }
  };
}
pub(crate) use cpu_instructions;

macro_rules! cpu_instructions_cb {
  ($self: expr, $op: expr) => {
    match($op) {
      _ => panic!("Invalid instruction")
    }
  };
}
pub(crate) use cpu_instructions_cb;
