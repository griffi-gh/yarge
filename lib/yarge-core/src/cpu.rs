mod reg;
mod instructions;
use instructions::*;
pub use reg::Registers;
use super::MMU;
use std::error::Error;
use crate::errors::InvalidInstrError;

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
  t: usize,

  #[cfg(feature = "breakpoints")]
  pub mmu_breakpoints: Box<[u8; 0xFFFF]>,
  #[cfg(feature = "breakpoints")]
  pub pc_breakpoints: Box<[bool; 0xFFFF]>,
}

impl CPU {
  pub fn new() -> Self {
    Self {
      reg: Registers::new(),
      mmu: MMU::new(),
      state: CPUState::Running,
      t: 0,

      #[cfg(feature = "breakpoints")]
      mmu_breakpoints: Box::new([0; 0xFFFF]),
      #[cfg(feature = "breakpoints")]
      pc_breakpoints: Box::new([false; 0xFFFF]),
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
  #[inline] fn fetch_signed(&mut self) -> i8 {
    //TODO remove this fn
    self.fetch() as i8 
  }

  #[inline] fn push(&mut self, value: u16) {
    self.reg.dec_sp(2);
    self.ww(self.reg.sp, value);
  }
  #[inline] fn pop(&mut self) -> u16 {
    let value = self.rw(self.reg.sp);
    self.reg.inc_sp(2);
    value
  }

  #[cfg(feature = "breakpoints")]
  fn check_mmu_breakpoints(&self, access_type: u8, addr: u16, value: Option<u8>) {
    let breakpoint_acc_type = self.mmu_breakpoints[addr as usize];
    let trip = breakpoint_acc_type & access_type;
    if trip != 0 {
      let value = if value.is_some() {
        let value = value.unwrap();
        format!("{value:#04X}")
      } else { "".to_string() };
      panic!("MMU Breakpoint hit {addr:#06X} {value}");
    }
  }

  #[inline] fn rb(&mut self, addr: u16) -> u8 {
    #[cfg(feature = "breakpoints")]
    self.check_mmu_breakpoints(0b01, addr, None);
    self.cycle();
    self.mmu.rb(addr)
  }
  #[inline] fn wb(&mut self, addr: u16, value: u8) {
    #[cfg(feature = "breakpoints")]
    self.check_mmu_breakpoints(0b10, addr, Some(value));
    self.cycle();
    self.mmu.wb(addr, value);
  }

  #[inline] fn rw(&mut self, addr: u16) -> u16 {
    self.rb(addr) as u16 | 
    ((self.rb(addr.wrapping_add(1)) as u16) << 8)
  }
  #[inline] fn ww(&mut self, addr: u16, value: u16) {
    self.wb(addr, (value & 0xFF) as u8);
    self.wb(addr.wrapping_add(1), (value >> 8) as u8);
  }

  fn cycle(&mut self) {
    self.t += 4;
    self.mmu.ppu.tick();
  }

  pub fn step(&mut self) -> Result<usize, Box<dyn Error>> {
    self.t = 0;
    if self.state == CPUState::Running {
      let mut op = self.fetch();
      if op != 0xCB { 
        cpu_instructions!(self, op)?;
      } else {
        op = self.fetch();
        cpu_instructions_cb!(self, op)?;
      }         
    } else {
      self.cycle();
    }
    #[cfg(debug_assertions)]
    assert!(self.t >= 4);
    Ok(self.t)
  }
}
