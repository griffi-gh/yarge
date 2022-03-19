mod reg;
mod instructions;
use instructions::*;
pub use reg::Registers;
use super::MMU;

#[derive(PartialEq)]
pub enum CPUState {
  Running,
  Halt,
  Stop
}

pub struct CPU {
  pub reg: Registers,
  pub mmu: MMU,
  pub state: CPUState,
  t: u32,
}

impl CPU {
  pub fn new() -> Self {
    Self {
      reg: Registers::new(),
      mmu: MMU::new(),
      state: CPUState::Running,
      t: 0,
    }
  }

  fn fetch(&mut self) -> u8 { 
    let op = self.rb(self.reg.pc);
    self.reg.inc_pc(1);
    return op
  }
  fn fetch_word(&mut self) -> u16 {
    let op = self.rw(self.reg.pc);
    self.reg.inc_pc(2);
    return op
  }
  #[inline]
  fn fetch_signed(&mut self) -> i8 {
    i8::from_ne_bytes([self.fetch()])
  }

  #[inline]
  fn push(&mut self, value: u16) {
    self.reg.dec_sp(2);
    self.ww(self.reg.sp, value);
  }
  fn pop(&mut self) -> u16 {
    let value = self.rw(self.reg.sp);
    self.reg.inc_sp(2);
    return value;
  }

  #[inline(always)]
  fn rb(&mut self, addr: u16) -> u8 {
    self.cycles(4);
    self.mmu.rb(addr)
  }
  #[inline(always)]
  fn wb(&mut self, addr: u16, value: u8) {
    self.cycles(4);
    self.mmu.wb(addr, value);
  }

  #[inline(always)]
  fn rw(&mut self, addr: u16) -> u16 {
    self.cycles(8);
    self.mmu.rw(addr)
  }
  #[inline(always)]
  fn ww(&mut self, addr: u16, value: u16) {
    self.cycles(8);
    self.mmu.ww(addr, value);
  }

  #[inline(always)]
  fn cycles(&mut self, cycles: u32) {
    self.t += cycles;
    self.tick_comp(cycles);
  }

  fn tick_comp(&mut self, t: u32) {
    self.t += t;
    self.mmu.ppu.tick(t);
  }

  pub fn step(&mut self) -> u32 {
    self.t = 0;
    if self.state == CPUState::Running {
      let mut op = self.fetch();
      if op != 0xCB { 
        cpu_instructions!(self, op);
      } else {
        op = self.fetch();
        cpu_instructions_cb!(self, op);
      }         
    } else {
      self.cycles(4);
    }
    return self.t;
  }
}
