pub use paste::paste;

//MAYBE Separate macro and instruction table?

macro_rules! panic_invalid_instruction {
  ($self: expr, $op: expr, $cb: expr) => {
    panic!(
      "Invalid or not yet implemented instruction{}{:#04X} at {:#06X}", 
      if $cb { " (CB) " } else { " " }, 
      $op, $self.reg.pc.wrapping_sub(1)
    )
  };
}
pub(crate) use panic_invalid_instruction;

macro_rules! ld_r_u8 {
  ($self: expr, $reg: ident) => { 
    let val = $self.fetch();
    paste! { 
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
}
pub(crate) use ld_r_u8;

macro_rules! ld_mhl_u8 {
  ($self: expr) => { 
    let val = $self.fetch();
    $self.wb($self.reg.hl(), val);
    $self.internal(4);
  };
}
pub(crate) use ld_mhl_u8;

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

macro_rules! pop_rr {
  ($self: expr, $reg: ident) => {
    let v = $self.pop();
    paste! {
      $self.reg.[<set_ $reg:lower>](v);
    }
  };
}
pub(crate) use pop_rr;

macro_rules! push_rr {
  ($self: expr, $reg: ident) => {
    $self.internal(4);
    paste! {
      $self.push($self.reg.[<$reg:lower>]());
    }
  };
}
pub(crate) use push_rr;

macro_rules! jp_u16 {
  ($self: expr) => {
    let to = $self.rw($self.reg.pc);
    $self.reg.pc = to;
    $self.internal(4);
  };
}
pub(crate) use jp_u16;

macro_rules! cond_jp_u16 {
  ($self: expr, $cond: ident) => {
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        let to = $self.rw($self.reg.pc);
        $self.reg.pc = to;
        $self.internal(4);
      } else {
        //simulate fetch timing without actually doing it
        $self.internal(8); 
        $self.reg.inc_pc(2);
      }
    }
  }
}
pub(crate) use cond_jp_u16;

macro_rules! call_u16 {
  ($self: expr) => {
    let to = $self.fetch_word();
    $self.internal(4);
    $self.push($self.reg.pc);
    $self.reg.pc = to;
  };
}
pub(crate) use call_u16;

macro_rules! ld_mhl_r {
  ($self: expr, $reg: ident) => {
    todo!(); //TODO LD (HL),R macro
  };
}
pub(crate) use ld_mhl_r;

macro_rules! ld_r_mhl {
  ($self: expr, $reg: ident) => {
    let v = $self.rb($self.reg.hl());
    paste!{
      $self.reg.[<set_ $reg:lower>](v);
    }
  };
}
pub(crate) use ld_r_mhl;

macro_rules! ihalt {
  ($self: expr) => {
    $self.state = CPUState::Halt;
  };
}
pub(crate) use ihalt;

macro_rules! cpu_instructions {
  ($self: expr, $op: expr) => {
    match($op) {
      0x00 => { /*IS A NO-OP*/ },               //NOP
      0x01 => { ld_rr_u16!($self, BC); },       //LD BC,u16
      0x02 => { ld_mrr_a!($self, BC); },        //LD (BC),A
      0x03 => { incdec_rr!($self, BC, add); }   //INC BC
      0x06 => { ld_r_u8!($self, B); }           //LD B,u8 
      0x0B => { incdec_rr!($self, BC, sub); }   //DEC BC
      0x0E => { ld_r_u8!($self, C); }           //LD C,u8 

      0x11 => { ld_rr_u16!($self, DE); },       //LD DE,u16
      0x12 => { ld_mrr_a!($self, DE); },        //LD (DE),A
      0x13 => { incdec_rr!($self, DE, add); },  //INC DE
      0x16 => { ld_r_u8!($self, D); }           //LD D,u8 
      0x1B => { incdec_rr!($self, DE, sub); }   //DEC DE
      0x1E => { ld_r_u8!($self, E); }           //LD E,u8 

      0x21 => { ld_rr_u16!($self, HL); },       //LD HL,u16
      0x22 => { ld_mhli_a!($self, add); },      //LD (HL+),A
      0x23 => { incdec_rr!($self, HL, add); },  //INC HL
      0x26 => { ld_r_u8!($self, H); }           //LD H,u8 
      0x2B => { incdec_rr!($self, HL, sub); }   //DEC HL
      0x2E => { ld_r_u8!($self, L); }           //LD L,u8 

      0x31 => { ld_rr_u16!($self, SP); },       //LD SP,u16
      0x32 => { ld_mhli_a!($self, sub); },      //LD (HL-),A
      0x33 => { incdec_rr!($self, SP, add); },  //INC SP
      0x36 => { ld_mhl_u8!($self); }            //LD (HL), u8
      0x3B => { incdec_rr!($self, SP, sub); },  //DEC SP
      0x3E => { ld_r_u8!($self, A); }           //LD A,u8 

      0x40 => { /*TODO Breakpoint */ }          //LD B,B
      0x41 => { ld_r_r!($self, B, C); }         //LD B,C
      0x42 => { ld_r_r!($self, B, D); }         //LD B,D
      0x43 => { ld_r_r!($self, B, E); }         //LD B,E
      0x44 => { ld_r_r!($self, B, H); }         //LD B,H
      0x45 => { ld_r_r!($self, B, L); }         //LD B,L
      0x46 => { ld_r_mhl!($self, B); }          //LD B,(HL)
      0x47 => { ld_r_r!($self, B, A); }         //LD B,A
      0x48 => { ld_r_r!($self, C, B); }         //LD C,B
      0x49 => { /*IS A NO-OP*/ }                //LD C,C
      0x4A => { ld_r_r!($self, C, D); }         //LD C,D
      0x4B => { ld_r_r!($self, C, E); }         //LD C,E
      0x4C => { ld_r_r!($self, C, H); }         //LD C,H
      0x4D => { ld_r_r!($self, C, L); }         //LD C,L
      0x4E => { ld_r_mhl!($self, C); }          //LD C,(HL)
      0x4F => { ld_r_r!($self, C, A); }         //LD C,A

      0x50 => { ld_r_r!($self, D, B); }         //LD D,B
      0x51 => { ld_r_r!($self, D, C); }         //LD D,C
      0x52 => { /*IS A NO-OP*/ }                //LD D,D
      0x53 => { ld_r_r!($self, D, E); }         //LD D,E
      0x54 => { ld_r_r!($self, D, H); }         //LD D,H
      0x55 => { ld_r_r!($self, D, L); }         //LD D,L
      0x56 => { ld_r_mhl!($self, D); }          //LD D,(HL)
      0x57 => { ld_r_r!($self, D, A); }         //LD D,A
      0x58 => { ld_r_r!($self, E, B); }         //LD E,B
      0x59 => { ld_r_r!($self, E, C); }         //LD E,C
      0x5A => { ld_r_r!($self, E, D); }         //LD E,D
      0x5B => { /*IS A NO-OP*/ }                //LD E,E
      0x5C => { ld_r_r!($self, E, H); }         //LD E,H
      0x5D => { ld_r_r!($self, E, L); }         //LD E,L
      0x5E => { ld_r_mhl!($self, E); }          //LD E,(HL)
      0x5F => { ld_r_r!($self, E, A); }         //LD E,A

      0x60 => { ld_r_r!($self, H, B); }         //LD H,B
      0x61 => { ld_r_r!($self, H, C); }         //LD H,C
      0x62 => { ld_r_r!($self, H, D); }         //LD H,D
      0x63 => { ld_r_r!($self, H, E); }         //LD H,E
      0x64 => { /*IS A NO-OP*/ }                //LD H,H
      0x65 => { ld_r_r!($self, H, L); }         //LD H,L
      0x66 => { ld_r_mhl!($self, H); }          //LD H,(HL)
      0x67 => { ld_r_r!($self, H, A); }         //LD H,A
      0x68 => { ld_r_r!($self, L, B); }         //LD L,B
      0x69 => { ld_r_r!($self, L, C); }         //LD L,C
      0x6A => { ld_r_r!($self, L, D); }         //LD L,D
      0x6B => { ld_r_r!($self, L, E); }         //LD L,E
      0x6C => { ld_r_r!($self, L, H); }         //LD L,H
      0x6D => { /*IS A NO-OP*/ }                //LD L,L
      0x6E => { ld_r_mhl!($self, L); }          //LD L,(HL)
      0x6F => { ld_r_r!($self, L, A); }         //LD L,A
      
      0x70 => { ld_mhl_r!($self, B); }          //LD (HL),B
      0x71 => { ld_mhl_r!($self, C); }          //LD (HL),C
      0x72 => { ld_mhl_r!($self, D); }          //LD (HL),D
      0x73 => { ld_mhl_r!($self, E); }          //LD (HL),E
      0x74 => { ld_mhl_r!($self, H); }          //LD (HL),H
      0x75 => { ld_mhl_r!($self, L); }          //LD (HL),L
      0x76 => { ihalt!($self); }                //HALT
      0x77 => { ld_mhl_r!($self, A); }          //LD (HL),A
      0x78 => { ld_r_r!($self, A, B); }         //LD A,B
      0x79 => { ld_r_r!($self, A, C); }         //LD A,C
      0x7A => { ld_r_r!($self, A, D); }         //LD A,D
      0x7B => { ld_r_r!($self, A, E); }         //LD A,E
      0x7C => { ld_r_r!($self, A, H); }         //LD A,H
      0x7D => { ld_r_r!($self, A, L); }         //LD A,L
      0x7F => { /*IS A NO-OP*/ }                //LD A,A

      0xC1 => { pop_rr!($self, BC); }           //POP BC
      0xC2 => { cond_jp_u16!($self, NZ); }      //JP NZ,u16
      0xC3 => { jp_u16!($self); }               //JP u16
      0xC5 => { push_rr!($self, BC); }          //PUSH BC
      0xCA => { cond_jp_u16!($self, Z); }       //JP Z,u16  
      0xCD => { call_u16!($self); }             //CALL u16

      0xD1 => { pop_rr!($self, DE); }           //POP DE
      0xD2 => { cond_jp_u16!($self, NC); }      //JP NC,u16
      0xD5 => { push_rr!($self, DE); }          //PUSH DE
      0xDA => { cond_jp_u16!($self, C); }       //JP C,u16

      0xE1 => { pop_rr!($self, HL); }           //POP HL
      0xE5 => { push_rr!($self, HL); }          //PUSH HL

      0xF1 => { pop_rr!($self, AF); }           //POP AF
      0xF5 => { push_rr!($self, AF); }          //PUSH AF

      _ => panic_invalid_instruction!($self, $op, false) 
    }
  };
}
pub(crate) use cpu_instructions;

macro_rules! cpu_instructions_cb {
  ($self: expr, $op: expr) => {
    match($op) {
      _ => panic_invalid_instruction!($self, $op, true) 
    }
  };
}
pub(crate) use cpu_instructions_cb;
