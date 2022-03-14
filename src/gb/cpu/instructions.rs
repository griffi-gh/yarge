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

macro_rules! cpu_instructions {
  ($self: expr, $op: expr) => {
    match($op) {
      0x00 => { },                            //NOP
      0x01 => { ld_rr_u16!($self, BC); },     //LD BC,u16
      0x02 => {  ld_mrr_a!($self, BC); },     //LD (BC),A
      0x03 => { incdec_rr!($self, BC, add); } //INC BC
      0x11 => { ld_rr_u16!($self, DE); },     //LD DE,u16
      0x12 => {  ld_mrr_a!($self, DE); },     //LD (DE),A
      0x21 => { ld_rr_u16!($self, HL); },     //LD HL,u16
      0x22 => { ld_mhli_a!($self, add); },    //LD (HL+),A
      0x31 => { ld_rr_u16!($self, SP); },     //LD SP,u16
      0x32 => { ld_mhli_a!($self, sub); },    //LD (HL+),A
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
