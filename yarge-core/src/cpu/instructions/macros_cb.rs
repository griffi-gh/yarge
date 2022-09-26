
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
      let v = $self.rb($self.reg.hl())?.rotate_left(4);
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

    let carry = val & 0x80 != 0;
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

    let carry = val & 0x80 != 0;
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

    let carry = val & 1 != 0;
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

    let carry = val & 1 != 0;
    let val = (val >> 1) | (($self.reg.f_c() as u8) << 7);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use rr_mhl;


//SLA
macro_rules! sla_r {
  ($self: expr, $r: ident) => {
    paste! {
      let val = $self.reg.[<$r:lower>]();
    }

    let carry = val & 0x80 != 0;
    let val = val << 1;
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    paste! {
      $self.reg.[<set_ $r:lower>](val);
    }
  }
} pub(crate) use sla_r;

macro_rules! sla_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;

    let carry = val & 0x80 != 0;
    let val = val << 1;
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use sla_mhl;

//SRA
macro_rules! sra_r {
  ($self: expr, $r: ident) => {
    paste! {
      let mut val = $self.reg.[<$r:lower>]();
    }

    let carry = val & 1 != 0;
    val >>= 1;
    if val & 0x40 != 0 {
      val |= 0x80;
    }
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    paste! {
      $self.reg.[<set_ $r:lower>](val);
    }
  }
} pub(crate) use sra_r;

macro_rules! sra_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let mut val = $self.rb(hl)?;

    let carry = val & 1 != 0;
    val >>= 1;
    if val & 0x40 != 0 {
      val |= 0x80;
    }
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use sra_mhl;

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

//RRC
macro_rules! rrc_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let val = $self.reg.[<$reg:lower>]();
    }

    let carry = val & 1 != 0;
    let val = val.rotate_right(1);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    paste! {
      $self.reg.[<set_ $reg:lower>](val);
    }
  }
} pub(crate) use rrc_r;

macro_rules! rrc_mhl {
  ($self: expr) => {
    let hl = $self.reg.hl();
    let val = $self.rb(hl)?;

    let carry = val & 1 != 0;
    let val = val.rotate_right(1);
    $self.reg.set_f_znhc(val == 0, false, false, carry);

    $self.wb(hl, val)?;
  }
} pub(crate) use rrc_mhl;

//SRL

macro_rules! srl_r {
  ($self: expr, $reg: ident) => {
    paste! {
      let val = $self.reg.[<$reg:lower>]();
    }
    let carry = val & 1 != 0;
    let val = val >> 1;
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
    let val = val >> 1;
    $self.reg.set_f_znhc(val == 0, false, false, carry);
    $self.wb(hl, val)?;
  };
} pub(crate) use srl_mhl;
