use crate::{
  cpu::{Cpu, Interrupt},
  consts::TIMER_CLOCK_MASKS,
};

pub struct Timers {
  pub tma: u8,
  div: u16,
  tima: u8,
  tima_reset_pending: bool,
  enable: bool,
  rate: u8,
  tima_inc: bool,
}
impl Timers {
  pub fn new() -> Self {
    Self {
      tma: 0,
      div: 0,
      tima: 0,
      tima_reset_pending: false,
      enable: false,
      rate: 0,
      tima_inc: false,
    }
  }

  pub fn get_div(&self) -> u8 {
    (self.div >> 8) as u8
  }
  pub fn reset_div(&mut self) {
    self.div = 0;
  }

  pub fn get_tima(&self) -> u8 {
    self.tima
  }
  pub fn set_tima(&mut self, value: u8) {
    self.tima = value;
    self.tima_reset_pending = false;
  }

  pub fn get_tac(&self) -> u8 {
    ((self.enable as u8) << 2) | self.rate
  }
  pub fn set_tac(&mut self, value: u8) {
    self.enable = value & 0b100 != 0;
    self.rate = value & 0b11;
  }

  pub fn tick(&mut self, iif: &mut u8) {
    if self.tima_reset_pending {
      self.tima_reset_pending = false;
      self.tima = self.tma;
      Cpu::set_interrupt(iif, Interrupt::Timer);
    }
    self.div = self.div.wrapping_add(4);
    let mask = TIMER_CLOCK_MASKS[(self.rate & 3) as usize];
    let div_bit = (self.div & mask) != 0;
    let cur_tima_inc = self.enable && div_bit;
    if self.tima_inc && !cur_tima_inc {
      let (tima, carry) = self.tima.overflowing_add(1);
      self.tima = tima;
      if carry {
        self.tima_reset_pending = true;
      }
    }
    self.tima_inc = cur_tima_inc;
  }
}
