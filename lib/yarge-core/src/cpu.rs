mod reg;
mod instructions;
use instructions::*;
pub use reg::Registers;
use super::MMU;
use crate::{
  Res, 
  errors::{
    InvalidInstrError,
    BreakpointHitError
  }
};

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

  #[cfg(feature = "breakpoints")]
  fn check_mmu_breakpoints(&self, access_type: u8, addr: u16, value: Option<u8>) -> Res<()> {
    let breakpoint_acc_type = self.mmu_breakpoints[addr as usize];
    let trip = breakpoint_acc_type & access_type;
    if trip != 0 {
      Err(Box::new(BreakpointHitError {
        is_pc: false,
        value: Some(value.unwrap_or(self.mmu.rb(addr))),
        addr,
      }))
    } else {
      Ok(())
    }
  }

  #[cfg(feature = "breakpoints")]
  fn check_pc_breakpoints(&self) -> Res<()> {
    let addr = self.reg.pc;
    if self.pc_breakpoints[addr as usize] {
      Err(Box::new(BreakpointHitError {
        is_pc: false,
        value: None,
        addr,
      }))
    } else {
      Ok(())
    }
  }

  #[inline] fn rb(&mut self, addr: u16) -> Res<u8> {
    #[cfg(feature = "breakpoints")]
    self.check_mmu_breakpoints(0b01, addr, None)?;
    self.cycle();
    Ok(self.mmu.rb(addr))
  }
  #[inline] fn wb(&mut self, addr: u16, value: u8) -> Res<()> {
    #[cfg(feature = "breakpoints")]
    self.check_mmu_breakpoints(0b10, addr, Some(value))?;
    self.cycle();
    self.mmu.wb(addr, value);
    Ok(())
  }

  #[inline] fn rw(&mut self, addr: u16) -> Res<u16> {
    Ok(
      self.rb(addr)? as u16 | 
      ((self.rb(addr.wrapping_add(1))? as u16) << 8)
    )
  }
  #[inline] fn ww(&mut self, addr: u16, value: u16) -> Res<()> {
    self.wb(addr, (value & 0xFF) as u8)?;
    self.wb(addr.wrapping_add(1), (value >> 8) as u8)?;
    Ok(())
  }

  fn fetch(&mut self) -> Res<u8> { 
    let op = self.rb(self.reg.pc)?;
    self.reg.inc_pc(1);
    Ok(op)
  }
  fn fetch_word(&mut self) -> Res<u16> {
    let op = self.rw(self.reg.pc)?;
    self.reg.inc_pc(2);
    Ok(op)
  }

  #[inline] fn push(&mut self, value: u16) -> Res<()> {
    self.reg.dec_sp(2);
    self.ww(self.reg.sp, value)?;
    Ok(())
  }
  #[inline] fn pop(&mut self) -> Res<u16> {
    let value = self.rw(self.reg.sp)?;
    self.reg.inc_sp(2);
    Ok(value)
  }

  fn cycle(&mut self) {
    self.t += 4;
    self.mmu.ppu.tick();
  }

  pub fn step(&mut self) -> Res<usize> {
    self.t = 0;
    #[cfg(feature = "breakpoints")] {
      self.check_pc_breakpoints()?;
    }
    if self.state == CPUState::Running {
      let mut op = self.fetch()?;
      if op != 0xCB { 
        cpu_instructions!(self, op)?;
      } else {
        op = self.fetch()?;
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
