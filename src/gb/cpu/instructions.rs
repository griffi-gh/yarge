pub use paste::paste;

macro_rules! ld_rr_u16 {
  ($self: expr, $reg: ident) => { paste! { 
    let val = $self.fetch_word();
    $self.reg.[<set_ $reg:lower>](val);
  }};
}
pub(crate) use ld_rr_u16;

macro_rules! cpu_instructions {
  ($self: expr, $op: expr) => {
    match($op) {
      0x00 => {},
      0x01 => { ld_rr_u16!($self, BC); },
      0x11 => { ld_rr_u16!($self, DE); },
      0x21 => { ld_rr_u16!($self, HL); },
      0x31 => { ld_rr_u16!($self, SP); },
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
