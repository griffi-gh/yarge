pub(crate) use paste::paste;

//WARNING: pls never use my macro approach.

//MAYBE Separate macro and instruction table?
//MAYBE Use functions w/enums instead of macros?

/*macro_rules! panic_invalid_instruction {
  ($self: expr, $op: expr, $cb: expr) => {
    panic!(
      "Invalid or not yet implemented instruction{}{:#04X} at {:#06X}", 
      if $cb { " (CB) " } else { " " }, 
      $op, $self.reg.pc.wrapping_sub(1)
    )
  };
} pub(crate) use panic_invalid_instruction;*/

macro_rules! ld_r_u8 {
  ($self: expr, $reg: ident) => { 
    let val = $self.fetch()?;
    paste! { 
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
} pub(crate) use ld_r_u8;

macro_rules! ld_mhl_u8 {
  ($self: expr) => { 
    let val = $self.fetch()?;
    $self.wb($self.reg.hl(), val)?;
  };
} pub(crate) use ld_mhl_u8;

macro_rules! ld_rr_u16 {
  ($self: expr, $reg: ident) => { 
    let val = $self.fetch_word()?;
    paste! { 
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
} pub(crate) use ld_rr_u16;

macro_rules! ld_mrr_a {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.reg.[<$reg:lower>]();
    } 
    $self.wb(v, $self.reg.a())?;
  };
} pub(crate) use ld_mrr_a;

macro_rules! ld_a_mrr {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.rb($self.reg.[<$reg:lower>]())?;
      $self.reg.set_a(v);
    }
  };
} pub(crate) use ld_a_mrr;

macro_rules! ld_a_mu16 {
  ($self: expr) => {
    let a = $self.fetch_word()?;
    let v = $self.rb(a)?;
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_mu16;

macro_rules! ld_mu16_a {
  ($self: expr) => {
    let a = $self.fetch_word()?;
    $self.wb(a, $self.reg.a())?;
  };
} pub(crate) use ld_mu16_a;

macro_rules! ld_mhli_a {
  ($self: expr, $inc: ident) => {
    let v = $self.reg.hl();
    paste! {
      $self.reg.set_hl($self.reg.hl().[<wrapping_ $inc:lower>](1));
    }
    $self.wb(v, $self.reg.a())?;
  };
} pub(crate) use ld_mhli_a;

macro_rules! ld_a_mhli {
  ($self: expr, $inc: ident) => {
    let v = $self.reg.hl();
    paste! {
      $self.reg.set_hl($self.reg.hl().[<wrapping_ $inc:lower>](1));
    }
    let v = $self.rb(v)?;
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_mhli;

macro_rules! incdec_rr {
  ($self: expr, $reg: ident, $inc: ident) => {
    paste! {
      $self.reg.[<set_ $reg:lower>](
        $self.reg.[<$reg:lower>]().[<wrapping_ $inc:lower>](1)
      );
    }
    $self.cycle();
  };
} pub(crate) use incdec_rr;

macro_rules! ld_r_r {
  ($self: expr, $a: ident, $b: ident) => {
    paste! {
      $self.reg.[<set_ $a:lower>](
        $self.reg.[<$b:lower>]()
      );
    }
  };
} pub(crate) use ld_r_r;

macro_rules! pop_rr {
  ($self: expr, $reg: ident) => {
    let v = $self.pop()?;
    paste! {
      $self.reg.[<set_ $reg:lower>](v);
    }
  };
} pub(crate) use pop_rr;

macro_rules! push_rr {
  ($self: expr, $reg: ident) => {
    $self.cycle();
    paste! {
      $self.push($self.reg.[<$reg:lower>]())?;
    }
  };
} pub(crate) use push_rr;

// JP u16

macro_rules! jp_u16 {
  ($self: expr) => {
    let to = $self.rw($self.reg.pc)?;
    $self.reg.pc = to;
    $self.cycle();
  };
} pub(crate) use jp_u16;

macro_rules! cond_jp_u16 {
  ($self: expr, $cond: ident) => {
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        let to = $self.rw($self.reg.pc)?;
        $self.reg.pc = to;
        $self.cycle();
      } else {
        //simulate fetch timing without actually doing it
        $self.cycle(); 
        $self.cycle();
        $self.reg.inc_pc(2);
      }
    }
  }
} pub(crate) use cond_jp_u16;

macro_rules! jp_hl {
  ($self: expr) => {
    $self.reg.pc = $self.reg.hl();
  };
} pub(crate) use jp_hl;

// CALL

macro_rules! call_u16 {
  ($self: expr) => {
    let to = $self.fetch_word()?;
    $self.cycle();
    $self.push($self.reg.pc)?;
    $self.reg.pc = to;
  };
} pub(crate) use call_u16;

macro_rules! call_u16_cond {
  ($self: expr, $cond: ident) => {
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        call_u16!($self);
      } else {
        //simulate fetch timing
        $self.cycle();
        $self.cycle();
        $self.reg.inc_pc(2);
      }
    }
  };
} pub(crate) use call_u16_cond;

// RET

macro_rules! ret {
  ($self: expr) => {
    $self.reg.pc = $self.pop()?;
    $self.cycle();
  } 
} pub(crate) use ret;

macro_rules! ret_cond {
  ($self: expr, $cond: ident) => {
    $self.cycle();
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        ret!($self);
      } else {
        //TODO simulate pop timing instead
        $self.pop()?; 
      }
    }
  };
} pub(crate) use ret_cond;


macro_rules! reti {
  ($self: expr) => {
    ret!($self);
    $self.enable_ime();
  } 
} pub(crate) use reti;

// RST 

macro_rules! rst {
  ($self: expr, $addr: expr) => {
    $self.cycle();
    $self.push($self.reg.pc)?;
    $self.reg.pc = $addr;
  };
} pub(crate) use rst;

//

macro_rules! ld_mhl_r {
  ($self: expr, $reg: ident) => {
    paste! {
      $self.wb($self.reg.hl(), $self.reg.[<$reg:lower>]())?;
    }
  };
} pub(crate) use ld_mhl_r;

macro_rules! ld_r_mhl {
  ($self: expr, $reg: ident) => {
    let v = $self.rb($self.reg.hl())?;
    paste!{
      $self.reg.[<set_ $reg:lower>](v);
    }
  };
} pub(crate) use ld_r_mhl;

macro_rules! cpu_halt {
  ($self: expr) => {
    $self.state = CpuState::Halt;
  };
} pub(crate) use cpu_halt;

macro_rules! cpu_stop {
  ($self: expr) => {
    //TODO realistic STOP
    $self.state = CpuState::Stop;
  };
} pub(crate) use cpu_stop;

macro_rules! inc_flags {
  ($self: expr, $v: expr, $r: expr) => {
    $self.reg.set_f_z($r == 0);
    $self.reg.set_f_n(false);
    $self.reg.set_f_h(($v & 0x0F) + 1 > 0x0F);
  }
} pub(crate) use inc_flags;

macro_rules! dec_flags {
  ($self: expr, $v: expr, $r: expr) => {
    $self.reg.set_f_z($r == 0);
    $self.reg.set_f_n(true);
    $self.reg.set_f_h(($v & 0x0F) == 0);
  }
} pub(crate) use dec_flags;

macro_rules! inc_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.reg.[<$reg:lower>]();
    }
    let r = v.wrapping_add(1);
    inc_flags!($self, v, r);
    paste! {
      $self.reg.[<set_ $reg:lower>](r);
    }
  };
} pub(crate) use inc_r;

macro_rules! dec_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.reg.[<$reg:lower>]();
    }
    let r = v.wrapping_sub(1);
    dec_flags!($self, v, r);
    paste! {
      $self.reg.[<set_ $reg:lower>](r);
    }
  };
} pub(crate) use dec_r;

macro_rules! inc_mhl {
  ($self: expr) => {
    let v = $self.rb($self.reg.hl())?;
    let r = v.wrapping_add(1);
    inc_flags!($self, v, r);
    $self.wb($self.reg.hl(), r)?;
  };
} pub(crate) use inc_mhl;

macro_rules! dec_mhl {
  ($self: expr) => {
    let v = $self.rb($self.reg.hl())?;
    let r = v.wrapping_sub(1);
    dec_flags!($self, v, r);
    $self.wb($self.reg.hl(), r)?;
  };
} pub(crate) use dec_mhl;

//ADD A

macro_rules! alu_add_a {
  ($self: expr, $b: expr) => {
    let a = $self.reg.a();
    let r = a.overflowing_add($b);
    $self.reg.set_f_znhc( //Z N H C
      r.0 == 0,
      false,
      (a & 0xF) + ($b & 0xF) > 0xF,
      r.1
    );
    $self.reg.set_a(r.0);
  };
} pub(crate) use alu_add_a;

macro_rules! add_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let b = $self.reg.[<$reg:lower>]();
    }
    alu_add_a!($self, b);
  };
} pub(crate) use add_a_r;

macro_rules! add_a_mhl {
  ($self: expr) => {
    let b = $self.rb($self.reg.hl())?;
    alu_add_a!($self, b);
  };
} pub(crate) use add_a_mhl;

macro_rules! add_a_u8 {
  ($self: expr) => {
    let b = $self.fetch()?;
    alu_add_a!($self, b);
  };
} pub(crate) use add_a_u8;

//SUB A

macro_rules! alu_sub_a {
  ($self: expr, $b: expr) => {
    let a = $self.reg.a();
    let r = a.overflowing_sub($b);
    $self.reg.set_f_znhc( //Z N H C
      r.0 == 0,
      true,
      (a & 0x0F) < ($b & 0x0F),
      r.1
    );
    $self.reg.set_a(r.0);
  };
} pub(crate) use alu_sub_a;

macro_rules! sub_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let b = $self.reg.[<$reg:lower>]();
    }
    alu_sub_a!($self, b);
  };
} pub(crate) use sub_a_r;

macro_rules! sub_a_mhl {
  ($self: expr) => {
    let b = $self.rb($self.reg.hl())?;
    alu_sub_a!($self, b);
  };
} pub(crate) use sub_a_mhl;

macro_rules! sub_a_u8 {
  ($self: expr) => {
    let b = $self.fetch()?;
    alu_sub_a!($self, b);
  };
} pub(crate) use sub_a_u8;

//CP A
macro_rules! alu_cp_a {
  ($self: expr, $b: expr) => {
    let a = $self.reg.a();
    $self.reg.set_f_znhc(
      a.wrapping_sub($b) == 0,
      true,
      ($b & 0xF) > (a & 0xF),
      $b > a
    );
  };
} pub(crate) use alu_cp_a;

macro_rules! cp_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let b = $self.reg.[<$reg:lower>]();
    }
    alu_cp_a!($self, b);
  };
} pub(crate) use cp_a_r;

macro_rules! cp_a_mhl {
  ($self: expr) => {
    let b = $self.rb($self.reg.hl())?;
    alu_cp_a!($self, b);
  };
} pub(crate) use cp_a_mhl;

macro_rules! cp_a_u8 {
  ($self: expr) => {
    let b = $self.fetch()?;
    alu_cp_a!($self, b);
  };
} pub(crate) use cp_a_u8;

//OR XOR, AND A

macro_rules! and_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let r = $self.reg.a() & $self.reg.[<$reg:lower>]();
    }
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, true, false);
  };
} pub(crate) use and_a_r;

macro_rules! and_a_mhl {
  ($self: expr) => {
    let r = $self.reg.a() & $self.rb($self.reg.hl())?;
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, true, false);
  };
} pub(crate) use and_a_mhl;

macro_rules! and_a_u8 {
  ($self: expr) => {
    let r = $self.reg.a() & $self.fetch()?;
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, true, false);
  };
} pub(crate) use and_a_u8;

macro_rules! or_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let r = $self.reg.a() | $self.reg.[<$reg:lower>]();
    }
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use or_a_r;

macro_rules! or_a_mhl {
  ($self: expr) => {
    let r = $self.reg.a() | $self.rb($self.reg.hl())?;
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use or_a_mhl;

macro_rules! or_a_u8 {
  ($self: expr) => {
    let r = $self.reg.a() | $self.fetch()?;
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use or_a_u8;

macro_rules! xor_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let r = $self.reg.a() ^ $self.reg.[<$reg:lower>]();
    }
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use xor_a_r;

macro_rules! xor_a_mhl {
  ($self: expr) => {
    let r = $self.reg.a() ^ $self.rb($self.reg.hl())?;
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use xor_a_mhl;

macro_rules! xor_a_u8 {
  ($self: expr) => {
    let r = $self.reg.a() ^ $self.fetch()?;
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use xor_a_u8;

//JR 

macro_rules! jr_i8 {
  ($self: expr) => {
    let v = $self.fetch()? as i8;
    $self.reg.inc_pc(v as u16);
    $self.cycle();
  }; //Works fine?
} pub(crate) use jr_i8;

macro_rules! jr_i8_cond {
  ($self: expr, $cond: ident) => {
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        let v = $self.fetch()? as i8;
        $self.reg.inc_pc(v as u16);
        $self.cycle();
      } else {
        //simulate fetch
        $self.cycle(); 
        $self.reg.inc_pc(1);
      }
    } //Works fine??
  };
} pub(crate) use jr_i8_cond;

macro_rules! add_sp_i8 {
  ($self: expr) => {
    let sp = $self.reg.sp();
    let fetch = $self.fetch()? as i8 as i16 as u16;
    let result = sp.wrapping_add(fetch);
    let op = sp ^ fetch ^ result;
    $self.reg.set_sp(result);
    $self.reg.set_f_znhc(
      false, false,
      op & 0x10 != 0,
      op & 0x100 != 0
    );
    //internal
    $self.cycle();
    $self.cycle();
  };
} pub(crate) use add_sp_i8;

macro_rules! ld_a_m_ff00_add_c {
  ($self: expr) => {
    let v = $self.rb(0xFF00 | ($self.reg.c() as u16))?;
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_m_ff00_add_c;

macro_rules! ld_m_ff00_add_c_a {
  ($self: expr) => {
    $self.wb(0xFF00 | ($self.reg.c() as u16), $self.reg.a())?;
  };
} pub(crate) use ld_m_ff00_add_c_a;


macro_rules! ld_a_m_ff00_add_u8 {
  ($self: expr) => {
    let f = $self.fetch()? as u16;
    let v = $self.rb(0xFF00 | f)?;
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_m_ff00_add_u8;

macro_rules! ld_m_ff00_add_u8_a {
  ($self: expr) => {
    let f = $self.fetch()? as u16;
    $self.wb(0xFF00 | f, $self.reg.a())?;
  };
} pub(crate) use ld_m_ff00_add_u8_a;

//RLA
macro_rules! rla {
  ($self: expr) => {
    let val = $self.reg.a();
    $self.reg.set_a((val << 1) | ($self.reg.f_c() as u8));
    $self.reg.set_f_znhc(false, false, false, val & 0x80 != 0);
  }
} pub(crate) use rla;

//RRA
macro_rules! rra {
  ($self: expr) => {
    let val = $self.reg.a();
    $self.reg.set_a((val >> 1) | (($self.reg.f_c() << 7) as u8));
    $self.reg.set_f_znhc(false, false, false, val & 0x80 != 0);
  }
} pub(crate) use rra;

macro_rules! daa {
  ($self: expr) => {
    //TEST does m >=0x60 work? Do i need c|| in the second if?
    let n = $self.reg.f_n();
    let h = $self.reg.f_h();
    let c = $self.reg.f_c();
    let a = $self.reg.a();
    let mut m: u8 = if c { 0x60 } else { 0 };
    if h | (!n && (a & 0xF) > 9) {
      m |= 0x06;
    }
    if !n && a > 0x99 {
      m |= 0x60;
    }
    let r = if n {
      a.wrapping_sub(m)
    } else {
      a.wrapping_add(m)
    };
    $self.reg.set_f_znhc(r == 0, n, false, m >= 0x60);
    $self.reg.set_a(r);
  };
} pub(crate) use daa;

macro_rules! cpl {
  ($self: expr) => {
    $self.reg.set_a(!$self.reg.a());
    $self.reg.set_f_n(true);
    $self.reg.set_f_h(true);
  };
} pub(crate) use cpl;

macro_rules! ei {
  ($self: expr) => {
    $self.enable_ime();
  }
} pub(crate) use ei;

macro_rules! di {
  ($self: expr) => {
    $self.disable_ime();
  }
} pub(crate) use di;

macro_rules! add_hl_rr {
  ($self: expr, $reg: ident) => {
    paste! {
      let rr = $self.reg.[<$reg:lower>]();
    }
    let hl = $self.reg.hl();
    $self.reg.set_f_n(false);
    $self.reg.set_f_h((hl & 0xFFF) + (rr & 0xFFF) > 0xFFF);
    let hl = hl.overflowing_add(rr);
    $self.reg.set_f_c(hl.1);
    $self.reg.set_hl(hl.0);
    $self.cycle();
  }
} pub(crate) use add_hl_rr;

macro_rules! cpu_instructions {
  ($self: expr, $op: expr) => {
    {
      match($op) {
        0x00 => { /*IS A NO-OP*/ },               //NOP
        0x01 => { ld_rr_u16!($self, BC); },       //LD BC,u16
        0x02 => { ld_mrr_a!($self, BC); },        //LD (BC),A
        0x03 => { incdec_rr!($self, BC, add); }   //INC BC
        0x04 => { inc_r!($self, B); }             //INC B
        0x05 => { dec_r!($self, B); }             //DEC B
        0x06 => { ld_r_u8!($self, B); }           //LD B,u8
        0x09 => { add_hl_rr!($self, BC); }        //ADD HL,BC
        0x0A => { ld_a_mrr!($self, BC); }         //LD A,(BC)
        0x0B => { incdec_rr!($self, BC, sub); }   //DEC BC
        0x0C => { inc_r!($self, C); }             //INC C
        0x0D => { dec_r!($self, C); }             //DEC C
        0x0E => { ld_r_u8!($self, C); }           //LD C,u8 

        0x10 => { cpu_stop!($self); }             //STOP
        0x11 => { ld_rr_u16!($self, DE); },       //LD DE,u16
        0x12 => { ld_mrr_a!($self, DE); },        //LD (DE),A
        0x13 => { incdec_rr!($self, DE, add); },  //INC DE
        0x14 => { inc_r!($self, D); }             //INC D
        0x15 => { dec_r!($self, D); }             //DEC D
        0x16 => { ld_r_u8!($self, D); }           //LD D,u8
        0x17 => { rla!($self); }                  //RLA
        0x18 => { jr_i8!($self); }                //JR i8
        0x19 => { add_hl_rr!($self, DE); }        //ADD HL,DE
        0x1A => { ld_a_mrr!($self, DE); }         //LD A,(DE)
        0x1B => { incdec_rr!($self, DE, sub); }   //DEC DE
        0x1C => { inc_r!($self, E); }             //INC E
        0x1D => { dec_r!($self, E); }             //DEC E
        0x1E => { ld_r_u8!($self, E); }           //LD E,u8 
        0x1F => { rra!($self); }                  //RRA

        0x20 => { jr_i8_cond!($self, NZ); }       //JR NZ, i8
        0x21 => { ld_rr_u16!($self, HL); },       //LD HL,u16
        0x22 => { ld_mhli_a!($self, add); },      //LD (HL+),A
        0x23 => { incdec_rr!($self, HL, add); },  //INC HL
        0x24 => { inc_r!($self, H); }             //INC H
        0x25 => { dec_r!($self, H); }             //DEC H
        0x26 => { ld_r_u8!($self, H); }           //LD H,u8
        0x27 => { daa!($self); }                  //DAA
        0x28 => { jr_i8_cond!($self, Z); }        //JR Z, i8 
        0x29 => { add_hl_rr!($self, HL); }        //ADD HL,HL
        0x2A => { ld_a_mhli!($self, add); }       //LD A,(HL+)
        0x2B => { incdec_rr!($self, HL, sub); }   //DEC HL
        0x2C => { inc_r!($self, L); }             //INC L
        0x2D => { dec_r!($self, L); }             //DEC L
        0x2E => { ld_r_u8!($self, L); }           //LD L,u8 
        0x2F => { cpl!($self); }                  //CPL

        0x30 => { jr_i8_cond!($self, NC); }       //JR NZ, i8
        0x31 => { ld_rr_u16!($self, SP); },       //LD SP,u16
        0x32 => { ld_mhli_a!($self, sub); },      //LD (HL-),A
        0x33 => { incdec_rr!($self, SP, add); },  //INC SP
        0x34 => { inc_mhl!($self); }              //INC (HL)
        0x35 => { dec_mhl!($self); }              //DEC (HL)
        0x36 => { ld_mhl_u8!($self); }            //LD (HL), u8
        0x38 => { jr_i8_cond!($self, C); }        //JR C, i8 
        0x39 => { add_hl_rr!($self, SP); }        //ADD HL,SP
        0x3A => { ld_a_mhli!($self, sub); }       //LD A,(HL-)
        0x3B => { incdec_rr!($self, SP, sub); },  //DEC SP
        0x3C => { inc_r!($self, A); }             //INC A
        0x3D => { dec_r!($self, A); }             //DEC A
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
        0x76 => { cpu_halt!($self); }             //HALT
        0x77 => { ld_mhl_r!($self, A); }          //LD (HL),A
        0x78 => { ld_r_r!($self, A, B); }         //LD A,B
        0x79 => { ld_r_r!($self, A, C); }         //LD A,C
        0x7A => { ld_r_r!($self, A, D); }         //LD A,D
        0x7B => { ld_r_r!($self, A, E); }         //LD A,E
        0x7C => { ld_r_r!($self, A, H); }         //LD A,H
        0x7D => { ld_r_r!($self, A, L); }         //LD A,L
        0x7E => { ld_r_mhl!($self, A); }          //LD A,(HL)
        0x7F => { /*IS A NO-OP*/ }                //LD A,A

        0x80 => { add_a_r!($self, B); }           //ADD A,B
        0x81 => { add_a_r!($self, C); }           //ADD A,C
        0x82 => { add_a_r!($self, D); }           //ADD A,D
        0x83 => { add_a_r!($self, E); }           //ADD A,E
        0x84 => { add_a_r!($self, H); }           //ADD A,H
        0x85 => { add_a_r!($self, L); }           //ADD A,L
        0x86 => { add_a_mhl!($self); }            //ADD A,(HL)
        0x87 => { add_a_r!($self, A); }           //ADD A,A

        0x90 => { sub_a_r!($self, B); }           //SUB A,B
        0x91 => { sub_a_r!($self, C); }           //SUB A,C
        0x92 => { sub_a_r!($self, D); }           //SUB A,D
        0x93 => { sub_a_r!($self, E); }           //SUB A,E
        0x94 => { sub_a_r!($self, H); }           //SUB A,H
        0x95 => { sub_a_r!($self, L); }           //SUB A,L
        0x96 => { sub_a_mhl!($self); }            //SUB A,(HL)
        0x97 => { sub_a_r!($self, A); }           //SUB A,A
        
        0xA0 => { and_a_r!($self, B); }           //AND A,B
        0xA1 => { and_a_r!($self, C); }           //AND A,C
        0xA2 => { and_a_r!($self, D); }           //AND A,D
        0xA3 => { and_a_r!($self, E); }           //AND A,E
        0xA4 => { and_a_r!($self, H); }           //AND A,H
        0xA5 => { and_a_r!($self, L); }           //AND A,L
        0xA6 => { and_a_mhl!($self); }            //AND A,(HL)
        0xA7 => { and_a_r!($self, A); }           //AND A,A
        0xA8 => { xor_a_r!($self, B); }           //XOR A,B
        0xA9 => { xor_a_r!($self, C); }           //XOR A,C
        0xAA => { xor_a_r!($self, D); }           //XOR A,D
        0xAB => { xor_a_r!($self, E); }           //XOR A,E
        0xAC => { xor_a_r!($self, H); }           //XOR A,H
        0xAD => { xor_a_r!($self, L); }           //XOR A,L
        0xAE => { xor_a_mhl!($self); }            //XOR A,(HL)
        0xAF => { xor_a_r!($self, A); }           //XOR A,A

        0xB0 => { or_a_r!($self, B); }            //OR A,B
        0xB1 => { or_a_r!($self, C); }            //OR A,C
        0xB2 => { or_a_r!($self, D); }            //OR A,D
        0xB3 => { or_a_r!($self, E); }            //OR A,E
        0xB4 => { or_a_r!($self, H); }            //OR A,H
        0xB5 => { or_a_r!($self, L); }            //OR A,L
        0xB6 => { or_a_mhl!($self); }             //OR A,(HL)
        0xB7 => { or_a_r!($self, A); }            //OR A,A
        0xB8 => { cp_a_r!($self, B); }            //CP A,B
        0xB9 => { cp_a_r!($self, C); }            //CP A,C
        0xBA => { cp_a_r!($self, D); }            //CP A,D
        0xBB => { cp_a_r!($self, E); }            //CP A,E
        0xBC => { cp_a_r!($self, H); }            //CP A,H
        0xBD => { cp_a_r!($self, L); }            //CP A,L
        0xBE => { cp_a_mhl!($self); }             //CP A,(HL)
        0xBF => { cp_a_r!($self, A); }            //CP A,A

        0xC0 => { ret_cond!($self, NZ); }         //RET NZ
        0xC1 => { pop_rr!($self, BC); }           //POP BC
        0xC2 => { cond_jp_u16!($self, NZ); }      //JP NZ,u16
        0xC3 => { jp_u16!($self); }               //JP u16
        0xC4 => { call_u16_cond!($self, NZ); }    //CALL NZ,u16
        0xC5 => { push_rr!($self, BC); }          //PUSH BC
        0xC6 => { add_a_u8!($self); }             //ADD A,u8
        0xC7 => { rst!($self, 0x00); }            //RST 00h
        0xC8 => { ret_cond!($self, Z); }          //RET Z
        0xC9 => { ret!($self); }                  //RET
        0xCA => { cond_jp_u16!($self, Z); }       //JP Z,u16
        0xCC => { call_u16_cond!($self, Z); }     //CALL Z,u16
        0xCD => { call_u16!($self); }             //CALL u16
        0xCF => { rst!($self, 0x08); }            //RST 08h

        0xD0 => { ret_cond!($self, NC); }         //RET NC
        0xD1 => { pop_rr!($self, DE); }           //POP DE
        0xD2 => { cond_jp_u16!($self, NC); }      //JP NC,u16
        0xD4 => { call_u16_cond!($self, NC); }    //CALL NZ,u16
        0xD5 => { push_rr!($self, DE); }          //PUSH DE
        0xD6 => { sub_a_u8!($self); }             //SUB A,u8
        0xD7 => { rst!($self, 0x10); }            //RST 10h
        0xD8 => { ret_cond!($self, C); }          //RET C
        0xD9 => { reti!($self); }                 //RETI
        0xDA => { cond_jp_u16!($self, C); }       //JP C,u16
        0xDC => { call_u16_cond!($self, C); }     //CALL C,u16
        0xDF => { rst!($self, 0x18); }            //RST 18h

        0xE0 => { ld_m_ff00_add_u8_a!($self); }   //LD (FFOO+u8),A
        0xE1 => { pop_rr!($self, HL); }           //POP HL
        0xE2 => { ld_m_ff00_add_c_a!($self); }    //LD (FF00+C),A
        0xE5 => { push_rr!($self, HL); }          //PUSH HL
        0xE6 => { and_a_u8!($self); }             //AND A,u8
        0xE7 => { rst!($self, 0x20); }            //RST 20h
        0xE8 => { add_sp_i8!($self); }            //ADD SP,i8
        0xE9 => { jp_hl!($self); }                //JP HL
        0xEA => { ld_mu16_a!($self); }            //LD (u16),A
        0xEE => { xor_a_u8!($self); }             //XOR A,u8
        0xEF => { rst!($self, 0x28); }            //RST 28h

        0xF0 => { ld_a_m_ff00_add_u8!($self); }   //LD A,(FF00+u8)
        0xF1 => { pop_rr!($self, AF); }           //POP AF
        0xF2 => { ld_a_m_ff00_add_c!($self); }    //LD A,(FF00+C)
        0xF3 => { di!($self); }                   //DI
        0xF5 => { push_rr!($self, AF); }          //PUSH AF
        0xF6 => { or_a_u8!($self); }              //OR A,u8
        0xF7 => { rst!($self, 0x30); }            //RST 30h
        0xFA => { ld_a_mu16!($self); }            //LD A,(u16)
        0xFB => { ei!($self); }                   //EI
        0xFE => { cp_a_u8!($self); }              //CP A,u8
        0xFF => { rst!($self, 0x38); }            //RST 38h

        _ => { 
          Err(YargeError::InvalidInstruction{
            is_cb: false,
            addr: $self.reg.pc.wrapping_sub(1),
            instr: $op
          })?;
        }
      }
    }
  };
} pub(crate) use cpu_instructions;

macro_rules! swap_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.reg.[<$reg:lower>]().rotate_left(4);
      $self.reg.set_f_znhc(v == 0, false, false, false);
      $self.reg.[<set_ $reg:lower>](v);
    }
  };
} pub(crate) use swap_r;

macro_rules! swap_mhl {
  ($self: expr) => {
    paste! {
      let v = $self.rb($self.reg.hl())?;
      $self.reg.set_f_znhc(v == 0, false, false, false);
      $self.wb($self.reg.hl(), v)?;
    }
  };
} pub(crate) use swap_mhl;

macro_rules! bit_r {
  ($self: expr, $bit: expr, $reg: ident) => {
    paste! {
      $self.reg.set_f_z(($self.reg.[<$reg:lower>]() & (1 << $bit)) == 0);
    }
    $self.reg.set_f_n(false);
    $self.reg.set_f_h(true);
  };
} pub(crate) use bit_r;

macro_rules! bit_mhl {
  ($self: expr, $bit: expr) => {
    let v = $self.rb($self.reg.hl())?;
    $self.reg.set_f_z((v & (1 << $bit)) == 0);
    $self.reg.set_f_n(false);
    $self.reg.set_f_h(true);
  };
} pub(crate) use bit_mhl;

macro_rules! res_r {
  ($self: expr, $bit: expr, $reg: ident) => {
    paste! {
      let val = $self.reg.[<$reg:lower>]();
      $self.reg.[<set_ $reg:lower>](val & !(1 << $bit));
    }
  };
} pub(crate) use res_r;

macro_rules! res_mhl {
  ($self: expr, $bit: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;
    $self.wb(hl, val & !(1 << $bit))?;
  };
} pub(crate) use res_mhl;

macro_rules! set_r {
  ($self: expr, $bit: expr, $reg: ident) => {
    paste! {
      let val = $self.reg.[<$reg:lower>]();
      $self.reg.[<set_ $reg:lower>](val | (1 << $bit));
    }
  };
} pub(crate) use set_r;

macro_rules! set_mhl {
  ($self: expr, $bit: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;
    $self.wb(hl, val | (1 << $bit))?;
  };
} pub(crate) use set_mhl;

// TODO instead of duplicating either:
/*
* add wrapper macros for reg and mhl ops 
* add macros for operations for example: rl!();
*/

//RL
macro_rules! rl_r {
  ($self: expr, $r: ident) => {
    paste! {
      let val = $self.reg.[<$r:lower>]();
    }

    let carry = val & 0x80 == 0x80;
    let val = (val << 1) | ($self.reg.f_c() as u8);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    paste! {
      $self.reg.[<set_ $r:lower>](val);
    }
  }
} pub(crate) use rl_r;

macro_rules! rl_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;

    let carry = val & 0x80 == 0x80;
    let val = (val << 1) | ($self.reg.f_c() as u8);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use rl_mhl;

//RR
macro_rules! rr_r {
  ($self: expr, $r: ident) => {
    paste! {
      let val = $self.reg.[<$r:lower>]();
    }

    let carry = val & 1 == 0x80;
    let val = (val >> 1) | (($self.reg.f_c() as u8) << 7);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    paste! {
      $self.reg.[<set_ $r:lower>](val);
    }
  }
} pub(crate) use rr_r;

macro_rules! rr_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;

    let carry = val & 1 == 0x80;
    let val = (val >> 1) | (($self.reg.f_c() as u8) << 7);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use rr_mhl;

//RLC
macro_rules! rlc_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let val = $self.reg.[<$reg:lower>]();
    }

    let carry = val & 0x80 != 0;
    let val = val.rotate_left(1);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    paste! {
      $self.reg.[<set_ $reg:lower>](val);
    }
  }
} pub(crate) use rlc_r;

macro_rules! rlc_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;

    let carry = val & 0x80 != 0;
    let val = val.rotate_left(1);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use rlc_mhl;

//SRL

macro_rules! srl_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let val = $self.reg.[<$reg:lower>]();
    }
    let carry = val & 1 != 0;
    let val = val << 1;
    $self.reg.set_f_znhc(val == 0, false, false, carry);
    paste! {
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
} pub(crate) use srl_r;

macro_rules! srl_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;
    let carry = val & 1 != 0;
    let val = val << 1;
    $self.reg.set_f_znhc(val == 0, false, false, carry);
    $self.wb(hl, val)?;
  };
} pub(crate) use srl_mhl;

macro_rules! cpu_instructions_cb {
  ($self: expr, $op: expr) => {
    {
      match($op) {
        0x00 => { rlc_r!($self, B); }             // RLC B
        0x01 => { rlc_r!($self, C); }             // RLC C
        0x02 => { rlc_r!($self, D); }             // RLC D
        0x03 => { rlc_r!($self, E); }             // RLC E
        0x04 => { rlc_r!($self, H); }             // RLC H
        0x05 => { rlc_r!($self, L); }             // RLC L
        0x06 => { rlc_mhl!($self); }              // RLC (HL)
        0x07 => { rlc_r!($self, A); }             // RLC A
        
        0x10 => { rl_r!($self, B); }              // RL B
        0x11 => { rl_r!($self, C); }              // RL C
        0x12 => { rl_r!($self, D); }              // RL D
        0x13 => { rl_r!($self, E); }              // RL E
        0x14 => { rl_r!($self, H); }              // RL H
        0x15 => { rl_r!($self, L); }              // RL L
        0x16 => { rl_mhl!($self); }               // RL (HL)
        0x17 => { rl_r!($self, A); }              // RL A
        0x18 => { rr_r!($self, B); }              // RR B
        0x19 => { rr_r!($self, C); }              // RR C
        0x1A => { rr_r!($self, D); }              // RR D
        0x1B => { rr_r!($self, E); }              // RR E
        0x1C => { rr_r!($self, H); }              // RR H
        0x1D => { rr_r!($self, L); }              // RR L
        0x1E => { rr_mhl!($self); }               // RR (HL)
        0x1F => { rr_r!($self, A); }              // RR A

        0x30 => { swap_r!($self, B); }            // SWAP B
        0x31 => { swap_r!($self, C); }            // SWAP C
        0x32 => { swap_r!($self, D); }            // SWAP D
        0x33 => { swap_r!($self, E); }            // SWAP E
        0x34 => { swap_r!($self, H); }            // SWAP H
        0x35 => { swap_r!($self, L); }            // SWAP L
        0x36 => { swap_mhl!($self); }             // SWAP (HL)
        0x37 => { swap_r!($self, A); }            // SWAP A
        0x38 => { srl_r!($self, B); }             // SRL B
        0x39 => { srl_r!($self, C); }             // SRL C
        0x3A => { srl_r!($self, D); }             // SRL D
        0x3B => { srl_r!($self, E); }             // SRL E
        0x3C => { srl_r!($self, H); }             // SRL H
        0x3D => { srl_r!($self, L); }             // SRL L
        0x3E => { srl_mhl!($self); }              // SRL (HL)
        0x3F => { srl_r!($self, A); }             // SRL A

        0x40 => { bit_r!($self, 0, B); }          // BIT 0,B
        0x41 => { bit_r!($self, 0, C); }          // BIT 0,C
        0x42 => { bit_r!($self, 0, D); }          // BIT 0,D
        0x43 => { bit_r!($self, 0, E); }          // BIT 0,E
        0x44 => { bit_r!($self, 0, H); }          // BIT 0,H
        0x45 => { bit_r!($self, 0, L); }          // BIT 0,L
        0x46 => { bit_mhl!($self, 0); }           // BIT 0,(HL)
        0x47 => { bit_r!($self, 0, A); }          // BIT 0,A
        0x48 => { bit_r!($self, 1, B); }          // BIT 1,B
        0x49 => { bit_r!($self, 1, C); }          // BIT 1,C
        0x4A => { bit_r!($self, 1, D); }          // BIT 1,D
        0x4B => { bit_r!($self, 1, E); }          // BIT 1,E
        0x4C => { bit_r!($self, 1, H); }          // BIT 1,H
        0x4D => { bit_r!($self, 1, L); }          // BIT 1,L
        0x4E => { bit_mhl!($self, 1); }           // BIT 1,(HL)
        0x4F => { bit_r!($self, 1, A); }          // BIT 1,A

        0x50 => { bit_r!($self, 2, B); }          // BIT 2,B
        0x51 => { bit_r!($self, 2, C); }          // BIT 2,C
        0x52 => { bit_r!($self, 2, D); }          // BIT 2,D
        0x53 => { bit_r!($self, 2, E); }          // BIT 2,E
        0x54 => { bit_r!($self, 2, H); }          // BIT 2,H
        0x55 => { bit_r!($self, 2, L); }          // BIT 2,L
        0x56 => { bit_mhl!($self, 2); }           // BIT 2,(HL)
        0x57 => { bit_r!($self, 2, A); }          // BIT 2,A
        0x58 => { bit_r!($self, 3, B); }          // BIT 3,B
        0x59 => { bit_r!($self, 3, C); }          // BIT 3,C
        0x5A => { bit_r!($self, 3, D); }          // BIT 3,D
        0x5B => { bit_r!($self, 3, E); }          // BIT 3,E
        0x5C => { bit_r!($self, 3, H); }          // BIT 3,H
        0x5D => { bit_r!($self, 3, L); }          // BIT 3,L
        0x5E => { bit_mhl!($self, 3); }           // BIT 3,(HL)
        0x5F => { bit_r!($self, 3, A); }          // BIT 3,A

        0x60 => { bit_r!($self, 4, B); }          // BIT 4,B
        0x61 => { bit_r!($self, 4, C); }          // BIT 4,C
        0x62 => { bit_r!($self, 4, D); }          // BIT 4,D
        0x63 => { bit_r!($self, 4, E); }          // BIT 4,E
        0x64 => { bit_r!($self, 4, H); }          // BIT 4,H
        0x65 => { bit_r!($self, 4, L); }          // BIT 4,L
        0x66 => { bit_mhl!($self, 4); }           // BIT 4,(HL)
        0x67 => { bit_r!($self, 4, A); }          // BIT 4,A
        0x68 => { bit_r!($self, 5, B); }          // BIT 5,B
        0x69 => { bit_r!($self, 5, C); }          // BIT 5,C
        0x6A => { bit_r!($self, 5, D); }          // BIT 5,D
        0x6B => { bit_r!($self, 5, E); }          // BIT 5,E
        0x6C => { bit_r!($self, 5, H); }          // BIT 5,H
        0x6D => { bit_r!($self, 5, L); }          // BIT 5,L
        0x6E => { bit_mhl!($self, 5); }           // BIT 5,(HL)
        0x6F => { bit_r!($self, 5, A); }          // BIT 5,A

        0x70 => { bit_r!($self, 6, B); }          // BIT 6,B
        0x71 => { bit_r!($self, 6, C); }          // BIT 6,C
        0x72 => { bit_r!($self, 6, D); }          // BIT 6,D
        0x73 => { bit_r!($self, 6, E); }          // BIT 6,E
        0x74 => { bit_r!($self, 6, H); }          // BIT 6,H
        0x75 => { bit_r!($self, 6, L); }          // BIT 6,L
        0x76 => { bit_mhl!($self, 6); }           // BIT 6,(HL)
        0x77 => { bit_r!($self, 6, A); }          // BIT 6,A
        0x78 => { bit_r!($self, 7, B); }          // BIT 7,B
        0x79 => { bit_r!($self, 7, C); }          // BIT 7,C
        0x7A => { bit_r!($self, 7, D); }          // BIT 7,D
        0x7B => { bit_r!($self, 7, E); }          // BIT 7,E
        0x7C => { bit_r!($self, 7, H); }          // BIT 7,H
        0x7D => { bit_r!($self, 7, L); }          // BIT 7,L
        0x7E => { bit_mhl!($self, 7); }           // BIT 7,(HL)
        0x7F => { bit_r!($self, 7, A); }          // BIT 7,A

        0x80 => { res_r!($self, 0, B); }          // RES 0,B
        0x81 => { res_r!($self, 0, C); }          // RES 0,C
        0x82 => { res_r!($self, 0, D); }          // RES 0,D
        0x83 => { res_r!($self, 0, E); }          // RES 0,E
        0x84 => { res_r!($self, 0, H); }          // RES 0,H
        0x85 => { res_r!($self, 0, L); }          // RES 0,L
        0x86 => { res_mhl!($self, 0); }           // RES 0,(HL)
        0x87 => { res_r!($self, 0, A); }          // RES 0,A
        0x88 => { res_r!($self, 1, B); }          // RES 1,B
        0x89 => { res_r!($self, 1, C); }          // RES 1,C
        0x8A => { res_r!($self, 1, D); }          // RES 1,D
        0x8B => { res_r!($self, 1, E); }          // RES 1,E
        0x8C => { res_r!($self, 1, H); }          // RES 1,H
        0x8D => { res_r!($self, 1, L); }          // RES 1,L
        0x8E => { res_mhl!($self, 1); }           // RES 1,(HL)
        0x8F => { res_r!($self, 1, A); }          // RES 1,A

        0x90 => { res_r!($self, 2, B); }          // RES 2,B
        0x91 => { res_r!($self, 2, C); }          // RES 2,C
        0x92 => { res_r!($self, 2, D); }          // RES 2,D
        0x93 => { res_r!($self, 2, E); }          // RES 2,E
        0x94 => { res_r!($self, 2, H); }          // RES 2,H
        0x95 => { res_r!($self, 2, L); }          // RES 2,L
        0x96 => { res_mhl!($self, 2); }           // RES 2,(HL)
        0x97 => { res_r!($self, 2, A); }          // RES 2,A
        0x98 => { res_r!($self, 3, B); }          // RES 3,B
        0x99 => { res_r!($self, 3, C); }          // RES 3,C
        0x9A => { res_r!($self, 3, D); }          // RES 3,D
        0x9B => { res_r!($self, 3, E); }          // RES 3,E
        0x9C => { res_r!($self, 3, H); }          // RES 3,H
        0x9D => { res_r!($self, 3, L); }          // RES 3,L
        0x9E => { res_mhl!($self, 3); }           // RES 3,(HL)
        0x9F => { res_r!($self, 3, A); }          // RES 3,A

        0xA0 => { res_r!($self, 4, B); }          // RES 4,B
        0xA1 => { res_r!($self, 4, C); }          // RES 4,C
        0xA2 => { res_r!($self, 4, D); }          // RES 4,D
        0xA3 => { res_r!($self, 4, E); }          // RES 4,E
        0xA4 => { res_r!($self, 4, H); }          // RES 4,H
        0xA5 => { res_r!($self, 4, L); }          // RES 4,L
        0xA6 => { res_mhl!($self, 4); }           // RES 4,(HL)
        0xA7 => { res_r!($self, 4, A); }          // RES 4,A
        0xA8 => { res_r!($self, 5, B); }          // RES 5,B
        0xA9 => { res_r!($self, 5, C); }          // RES 5,C
        0xAA => { res_r!($self, 5, D); }          // RES 5,D
        0xAB => { res_r!($self, 5, E); }          // RES 5,E
        0xAC => { res_r!($self, 5, H); }          // RES 5,H
        0xAD => { res_r!($self, 5, L); }          // RES 5,L
        0xAE => { res_mhl!($self, 5); }           // RES 5,(HL)
        0xAF => { res_r!($self, 5, A); }          // RES 5,A

        0xB0 => { res_r!($self, 6, B); }          // RES 6,B
        0xB1 => { res_r!($self, 6, C); }          // RES 6,C
        0xB2 => { res_r!($self, 6, D); }          // RES 6,D
        0xB3 => { res_r!($self, 6, E); }          // RES 6,E
        0xB4 => { res_r!($self, 6, H); }          // RES 6,H
        0xB5 => { res_r!($self, 6, L); }          // RES 6,L
        0xB6 => { res_mhl!($self, 6); }           // RES 6,(HL)
        0xB7 => { res_r!($self, 6, A); }          // RES 6,A
        0xB8 => { res_r!($self, 7, B); }          // RES 7,B
        0xB9 => { res_r!($self, 7, C); }          // RES 7,C
        0xBA => { res_r!($self, 7, D); }          // RES 7,D
        0xBB => { res_r!($self, 7, E); }          // RES 7,E
        0xBC => { res_r!($self, 7, H); }          // RES 7,H
        0xBD => { res_r!($self, 7, L); }          // RES 7,L
        0xBE => { res_mhl!($self, 7); }           // RES 7,(HL)
        0xBF => { res_r!($self, 7, A); }          // RES 7,A

        0xC0 => { set_r!($self, 0, B); }          // SET 0,B
        0xC1 => { set_r!($self, 0, C); }          // SET 0,C
        0xC2 => { set_r!($self, 0, D); }          // SET 0,D
        0xC3 => { set_r!($self, 0, E); }          // SET 0,E
        0xC4 => { set_r!($self, 0, H); }          // SET 0,H
        0xC5 => { set_r!($self, 0, L); }          // SET 0,L
        0xC6 => { set_mhl!($self, 0); }           // SET 0,(HL)
        0xC7 => { set_r!($self, 0, A); }          // SET 0,A
        0xC8 => { set_r!($self, 1, B); }          // SET 1,B
        0xC9 => { set_r!($self, 1, C); }          // SET 1,C
        0xCA => { set_r!($self, 1, D); }          // SET 1,D
        0xCB => { set_r!($self, 1, E); }          // SET 1,E
        0xCC => { set_r!($self, 1, H); }          // SET 1,H
        0xCD => { set_r!($self, 1, L); }          // SET 1,L
        0xCE => { set_mhl!($self, 1); }           // SET 1,(HL)
        0xCF => { set_r!($self, 1, A); }          // SET 1,A

        0xD0 => { set_r!($self, 2, B); }          // SET 2,B
        0xD1 => { set_r!($self, 2, C); }          // SET 2,C
        0xD2 => { set_r!($self, 2, D); }          // SET 2,D
        0xD3 => { set_r!($self, 2, E); }          // SET 2,E
        0xD4 => { set_r!($self, 2, H); }          // SET 2,H
        0xD5 => { set_r!($self, 2, L); }          // SET 2,L
        0xD6 => { set_mhl!($self, 2); }           // SET 2,(HL)
        0xD7 => { set_r!($self, 2, A); }          // SET 2,A
        0xD8 => { set_r!($self, 3, B); }          // SET 3,B
        0xD9 => { set_r!($self, 3, C); }          // SET 3,C
        0xDA => { set_r!($self, 3, D); }          // SET 3,D
        0xDB => { set_r!($self, 3, E); }          // SET 3,E
        0xDC => { set_r!($self, 3, H); }          // SET 3,H
        0xDD => { set_r!($self, 3, L); }          // SET 3,L
        0xDE => { set_mhl!($self, 3); }           // SET 3,(HL)
        0xDF => { set_r!($self, 3, A); }          // SET 3,A

        0xE0 => { set_r!($self, 4, B); }          // SET 4,B
        0xE1 => { set_r!($self, 4, C); }          // SET 4,C
        0xE2 => { set_r!($self, 4, D); }          // SET 4,D
        0xE3 => { set_r!($self, 4, E); }          // SET 4,E
        0xE4 => { set_r!($self, 4, H); }          // SET 4,H
        0xE5 => { set_r!($self, 4, L); }          // SET 4,L
        0xE6 => { set_mhl!($self, 4); }           // SET 4,(HL)
        0xE7 => { set_r!($self, 4, A); }          // SET 4,A
        0xE8 => { set_r!($self, 5, B); }          // SET 5,B
        0xE9 => { set_r!($self, 5, C); }          // SET 5,C
        0xEA => { set_r!($self, 5, D); }          // SET 5,D
        0xEB => { set_r!($self, 5, E); }          // SET 5,E
        0xEC => { set_r!($self, 5, H); }          // SET 5,H
        0xED => { set_r!($self, 5, L); }          // SET 5,L
        0xEE => { set_mhl!($self, 5); }           // SET 5,(HL)
        0xEF => { set_r!($self, 5, A); }          // SET 5,A

        0xF0 => { set_r!($self, 6, B); }          // SET 6,B
        0xF1 => { set_r!($self, 6, C); }          // SET 6,C
        0xF2 => { set_r!($self, 6, D); }          // SET 6,D
        0xF3 => { set_r!($self, 6, E); }          // SET 6,E
        0xF4 => { set_r!($self, 6, H); }          // SET 6,H
        0xF5 => { set_r!($self, 6, L); }          // SET 6,L
        0xF6 => { set_mhl!($self, 6); }           // SET 6,(HL)
        0xF7 => { set_r!($self, 6, A); }          // SET 6,A
        0xF8 => { set_r!($self, 7, B); }          // SET 7,B
        0xF9 => { set_r!($self, 7, C); }          // SET 7,C
        0xFA => { set_r!($self, 7, D); }          // SET 7,D
        0xFB => { set_r!($self, 7, E); }          // SET 7,E
        0xFC => { set_r!($self, 7, H); }          // SET 7,H
        0xFD => { set_r!($self, 7, L); }          // SET 7,L
        0xFE => { set_mhl!($self, 7); }           // SET 7,(HL)
        0xFF => { set_r!($self, 7, A); }          // SET 7,A

        _ => { 
          Err(YargeError::InvalidInstruction {
            is_cb: true,
            addr: $self.reg.pc.wrapping_sub(1),
            instr: $op
          })?;
        }
      }
    }
  };
} pub(crate) use cpu_instructions_cb;
