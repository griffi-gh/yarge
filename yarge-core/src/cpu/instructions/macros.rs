pub(crate) use paste::paste;

macro_rules! ld_b_b {
  ($self: expr) => {
    #[cfg(feature = "dbg-breakpoint-on-ld-b-b")] {
      Err($crate::YargeError::LdBreakpoint {
        addr: $self.reg.pc.wrapping_sub(1)
      })?;
    }
  }
} pub(crate) use ld_b_b;

macro_rules! ld_r_u8 {
  ($self: expr, $reg: ident) => { 
    let val = $self.fetch();
    paste! { 
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
} pub(crate) use ld_r_u8;

macro_rules! ld_mhl_u8 {
  ($self: expr) => { 
    let val = $self.fetch();
    $self.wb($self.reg.hl(), val);
  };
} pub(crate) use ld_mhl_u8;

macro_rules! ld_rr_u16 {
  ($self: expr, $reg: ident) => { 
    let val = $self.fetch_word();
    paste! { 
      $self.reg.[<set_ $reg:lower>](val);
    }
  };
} pub(crate) use ld_rr_u16;

macro_rules! ld_sp_hl {
  ($self: expr) => { 
    $self.reg.sp = $self.reg.hl();
    $self.cycle();
  };
} pub(crate) use ld_sp_hl;

macro_rules! ld_mrr_a {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.reg.[<$reg:lower>]();
    } 
    $self.wb(v, $self.reg.a());
  };
} pub(crate) use ld_mrr_a;

macro_rules! ld_a_mrr {
  ($self: expr, $reg: ident) => {
    paste! {
      let v = $self.rb($self.reg.[<$reg:lower>]());
      $self.reg.set_a(v);
    }
  };
} pub(crate) use ld_a_mrr;

macro_rules! ld_a_mu16 {
  ($self: expr) => {
    let a = $self.fetch_word();
    let v = $self.rb(a);
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_mu16;

macro_rules! ld_mu16_a {
  ($self: expr) => {
    let a = $self.fetch_word();
    $self.wb(a, $self.reg.a());
  };
} pub(crate) use ld_mu16_a;

macro_rules! ld_mu16_sp {
  ($self: expr) => {
    let a = $self.fetch_word();
    $self.ww(a, $self.reg.sp);
  };
} pub(crate) use ld_mu16_sp;

macro_rules! ld_mhli_a {
  ($self: expr, $inc: ident) => {
    let v = $self.reg.hl();
    paste! {
      $self.reg.set_hl($self.reg.hl().[<wrapping_ $inc:lower>](1));
    }
    $self.wb(v, $self.reg.a());
  };
} pub(crate) use ld_mhli_a;

macro_rules! ld_a_mhli {
  ($self: expr, $inc: ident) => {
    let v = $self.reg.hl();
    paste! {
      $self.reg.set_hl($self.reg.hl().[<wrapping_ $inc:lower>](1));
    }
    let v = $self.rb(v);
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
    let v = $self.pop();
    paste! {
      $self.reg.[<set_ $reg:lower>](v);
    }
  };
} pub(crate) use pop_rr;

macro_rules! push_rr {
  ($self: expr, $reg: ident) => {
    $self.cycle();
    paste! {
      $self.push($self.reg.[<$reg:lower>]());
    }
  };
} pub(crate) use push_rr;

// JP u16

macro_rules! jp_u16 {
  ($self: expr) => {
    let to = $self.rw($self.reg.pc);
    $self.reg.pc = to;
    $self.cycle();
  };
} pub(crate) use jp_u16;

macro_rules! cond_jp_u16 {
  ($self: expr, $cond: ident) => {
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        let to = $self.rw($self.reg.pc);
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
    let to = $self.fetch_word();
    $self.cycle();
    $self.push($self.reg.pc);
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
    $self.reg.pc = $self.pop();
    $self.cycle();
  } 
} pub(crate) use ret;

macro_rules! ret_cond {
  ($self: expr, $cond: ident) => {
    $self.cycle();
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        ret!($self);
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
    $self.push($self.reg.pc);
    $self.reg.pc = $addr;
  };
} pub(crate) use rst;

//

macro_rules! ld_mhl_r {
  ($self: expr, $reg: ident) => {
    paste! {
      $self.wb($self.reg.hl(), $self.reg.[<$reg:lower>]());
    }
  };
} pub(crate) use ld_mhl_r;

macro_rules! ld_r_mhl {
  ($self: expr, $reg: ident) => {
    let v = $self.rb($self.reg.hl());
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
    $self.reg.inc_pc(1);
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
    let v = $self.rb($self.reg.hl());
    let r = v.wrapping_add(1);
    inc_flags!($self, v, r);
    $self.wb($self.reg.hl(), r);
  };
} pub(crate) use inc_mhl;

macro_rules! dec_mhl {
  ($self: expr) => {
    let v = $self.rb($self.reg.hl());
    let r = v.wrapping_sub(1);
    dec_flags!($self, v, r);
    $self.wb($self.reg.hl(), r);
  };
} pub(crate) use dec_mhl;

//ADD A

macro_rules! alu_add_a {
  ($self: expr, $b: expr) => {
    let a = $self.reg.a();
    let (result, carry) = a.overflowing_add($b);
    $self.reg.set_f_znhc( //Z N H C
      result == 0,
      false,
      (a & 0xF) + ($b & 0xF) > 0xF,
      carry
    );
    $self.reg.set_a(result);
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
    let b = $self.rb($self.reg.hl());
    alu_add_a!($self, b);
  };
} pub(crate) use add_a_mhl;

macro_rules! add_a_u8 {
  ($self: expr) => {
    let b = $self.fetch();
    alu_add_a!($self, b);
  };
} pub(crate) use add_a_u8;

//ADC A

macro_rules! alu_adc_a {
  ($self: expr, $b: expr) => {
    let c = $self.reg.f_c() as u8;
    let a = $self.reg.a();
    let (result, carry) = {
      let r0 = a.overflowing_add($b);
      let r1 = r0.0.overflowing_add(c);
      (r1.0, r0.1 || r1.1)
    };
    $self.reg.set_f_znhc(
      result == 0,
      false,
      ((a & 0xF) + ($b & 0xF) + c) > 0xF,
      carry
    );
    $self.reg.set_a(result);
  }
} pub(crate) use alu_adc_a;

macro_rules! adc_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let b = $self.reg.[<$reg:lower>]();
    }
    alu_adc_a!($self, b);
  };
} pub(crate) use adc_a_r;

macro_rules! adc_a_mhl {
  ($self: expr) => {
    let b = $self.rb($self.reg.hl());
    alu_adc_a!($self, b);
  };
} pub(crate) use adc_a_mhl;

macro_rules! adc_a_u8 {
  ($self: expr) => {
    let b = $self.fetch();
    alu_adc_a!($self, b);
  };
} pub(crate) use adc_a_u8;

//ADC A

macro_rules! alu_sbc_a {
  ($self: expr, $b: expr) => {
    let c = $self.reg.f_c() as u8;
    let a = $self.reg.a();
    let (result, carry) = {
      let r0 = a.overflowing_sub($b);
      let r1 = r0.0.overflowing_sub(c);
      (r1.0, r0.1 || r1.1)
    };
    $self.reg.set_f_znhc(
      result == 0,
      true,
      (a & 0xf).wrapping_sub($b & 0xf).wrapping_sub(c) & 0x10 != 0,
      carry
    );
    $self.reg.set_a(result);
  }
} pub(crate) use alu_sbc_a;

macro_rules! sbc_a_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let b = $self.reg.[<$reg:lower>]();
    }
    alu_sbc_a!($self, b);
  };
} pub(crate) use sbc_a_r;

macro_rules! sbc_a_mhl {
  ($self: expr) => {
    let b = $self.rb($self.reg.hl());
    alu_sbc_a!($self, b);
  };
} pub(crate) use sbc_a_mhl;

macro_rules! sbc_a_u8 {
  ($self: expr) => {
    let b = $self.fetch();
    alu_sbc_a!($self, b);
  };
} pub(crate) use sbc_a_u8;

//SUB A

macro_rules! alu_sub_a {
  ($self: expr, $b: expr) => {
    let a = $self.reg.a();
    let (result, carry) = a.overflowing_sub($b);
    $self.reg.set_f_znhc( //Z N H C
      result == 0,
      true,
      (a & 0x0F) < ($b & 0x0F),
      carry
    );
    $self.reg.set_a(result);
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
    let b = $self.rb($self.reg.hl());
    alu_sub_a!($self, b);
  };
} pub(crate) use sub_a_mhl;

macro_rules! sub_a_u8 {
  ($self: expr) => {
    let b = $self.fetch();
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
    let b = $self.rb($self.reg.hl());
    alu_cp_a!($self, b);
  };
} pub(crate) use cp_a_mhl;

macro_rules! cp_a_u8 {
  ($self: expr) => {
    let b = $self.fetch();
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
    let r = $self.reg.a() & $self.rb($self.reg.hl());
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, true, false);
  };
} pub(crate) use and_a_mhl;

macro_rules! and_a_u8 {
  ($self: expr) => {
    let r = $self.reg.a() & $self.fetch();
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
    let r = $self.reg.a() | $self.rb($self.reg.hl());
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use or_a_mhl;

macro_rules! or_a_u8 {
  ($self: expr) => {
    let r = $self.reg.a() | $self.fetch();
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

macro_rules! xor_a_a {
  ($self: expr) => {
    $self.reg.set_a(0);
    $self.reg.set_f_znhc(true, false, false, false);
  };
} pub(crate) use xor_a_a;

macro_rules! xor_a_mhl {
  ($self: expr) => {
    let r = $self.reg.a() ^ $self.rb($self.reg.hl());
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use xor_a_mhl;

macro_rules! xor_a_u8 {
  ($self: expr) => {
    let r = $self.reg.a() ^ $self.fetch();
    $self.reg.set_a(r);
    $self.reg.set_f_znhc(r == 0, false, false, false);
  };
} pub(crate) use xor_a_u8;

//JR 

macro_rules! jr_i8 {
  ($self: expr) => {
    let v = $self.fetch() as i8;
    $self.reg.inc_pc(v as u16);
    $self.cycle();
  }; //Works fine?
} pub(crate) use jr_i8;

macro_rules! jr_i8_cond {
  ($self: expr, $cond: ident) => {
    paste! {
      if $self.reg.[<f_ $cond:lower>]() {
        let v = $self.fetch() as i8;
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

macro_rules! sp_i8 {
  ($self: expr) => {{
    let sp = $self.reg.sp();
    let fetch = $self.fetch() as i8 as i16 as u16;
    let result = sp.wrapping_add(fetch);
    let op = sp ^ fetch ^ result;
    $self.reg.set_f_znhc(
      false, false,
      op & 0x10 != 0,
      op & 0x100 != 0
    );
    result
  }}
} pub(crate) use sp_i8;

macro_rules! add_sp_i8 {
  ($self: expr) => {
    let v = sp_i8!($self);
    $self.reg.set_sp(v);
    //internal
    $self.cycle();
    $self.cycle();
  };
} pub(crate) use add_sp_i8;

macro_rules! ld_hl_sp_i8 {
  ($self: expr) => {
    let v = sp_i8!($self);
    $self.reg.set_hl(v);
    //internal
    $self.cycle();
  };
} pub(crate) use ld_hl_sp_i8;

macro_rules! ld_a_m_ff00_add_c {
  ($self: expr) => {
    let v = $self.rb(0xFF00 | ($self.reg.c() as u16));
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_m_ff00_add_c;

macro_rules! ld_m_ff00_add_c_a {
  ($self: expr) => {
    $self.wb(0xFF00 | ($self.reg.c() as u16), $self.reg.a());
  };
} pub(crate) use ld_m_ff00_add_c_a;


macro_rules! ld_a_m_ff00_add_u8 {
  ($self: expr) => {
    let f = $self.fetch() as u16;
    let v = $self.rb(0xFF00 | f);
    $self.reg.set_a(v);
  };
} pub(crate) use ld_a_m_ff00_add_u8;

macro_rules! ld_m_ff00_add_u8_a {
  ($self: expr) => {
    let f = $self.fetch() as u16;
    $self.wb(0xFF00 | f, $self.reg.a());
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
    $self.reg.set_a((val >> 1) | (($self.reg.f_c() as u8) << 7));
    $self.reg.set_f_znhc(false, false, false, val & 1 != 0);
  }
} pub(crate) use rra;

//RLCA
macro_rules! rlca {
  ($self: expr) => {
    let val = $self.reg.a();
    $self.reg.set_a(val.rotate_left(1));
    $self.reg.set_f_znhc(false, false, false, val & 0x80 != 0);
  }
} pub(crate) use rlca;

//RRCA
macro_rules! rrca {
  ($self: expr) => {
    let val = $self.reg.a();
    $self.reg.set_a(val.rotate_right(1));
    $self.reg.set_f_znhc(false, false, false, val & 1 != 0);
  }
} pub(crate) use rrca;


macro_rules! daa {
  ($self: expr) => {
    let n = $self.reg.f_n();
    let h = $self.reg.f_h();
    let c = $self.reg.f_c();
    let a = $self.reg.a();
    //get the m thing
    let mut m: u8 = 0;
    if h || (!n && (a & 0xF) > 9) {
      m |= 0x06;
    }
    if c || (!n && a > 0x99) {
      m |= 0x60;
    };
    //apply it to a
    let r = if n {
      a.wrapping_sub(m)
    } else {
      a.wrapping_add(m)
    };
    //set things
    $self.reg.set_f_znhc(r == 0, n, false, m >= 0x60);
    $self.reg.set_a(r);
  };
} pub(crate) use daa;

macro_rules! scf {
  ($self: expr) => {
    $self.reg.set_f_c(true);
    $self.reg.set_f_n(false);
    $self.reg.set_f_h(false);
  };
} pub(crate) use scf;

macro_rules! ccf {
  ($self: expr) => {
    $self.reg.set_f_c(!$self.reg.f_c());
    $self.reg.set_f_n(false);
    $self.reg.set_f_h(false);
  };
} pub(crate) use ccf;

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
