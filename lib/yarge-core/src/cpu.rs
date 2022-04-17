mod reg;
mod instructions;
use instructions::*;
pub use reg::Registers;
use crate::{ Mmu, Res, YargeError, consts::INT_JMP_VEC };

#[derive(PartialEq)]
pub enum CpuState {
  Running,
  Halt,
  Stop
}

#[repr(u8)]
pub enum Interrupt {
  VBlank  = 0,
  Stat    = 1,
  Timer   = 2,
  Serial  = 3,
  Joypad  = 4,
}

pub struct Cpu {
  pub reg: Registers,
  pub mmu: Mmu,
  pub state: CpuState,
  ime_pending: bool,
  ime: bool,
  t: usize,

  #[cfg(feature = "breakpoints")]
  pub mmu_breakpoints: Box<[u8; 0x10000]>,
  #[cfg(feature = "breakpoints")]
  pub pc_breakpoints: Box<[bool; 0x10000]>,
}

impl Cpu {
  pub fn new() -> Self {
    Self {
      reg: Registers::new(),
      mmu: Mmu::new(),
      state: CpuState::Running,
      ime_pending: false,
      ime: false,
      t: 0,

      #[cfg(feature = "breakpoints")]
      mmu_breakpoints: Box::new([0; 0x10000]),
      #[cfg(feature = "breakpoints")]
      pc_breakpoints: Box::new([false; 0x10000]),
    }
  }

  #[cfg(feature = "breakpoints")]
  fn check_mmu_breakpoints(&self, access_type: u8, addr: u16, value: Option<u8>) -> Res<()> {
    let breakpoint_acc_type = self.mmu_breakpoints[addr as usize];
    let trip = breakpoint_acc_type & access_type;
    if trip != 0 {
      Err(YargeError::MmuBreakpoint {
        is_write: access_type & 0b10 != 0,
        value: value.unwrap_or(self.mmu.rb(addr)),
        addr,
      })
    } else {
      Ok(())
    }
  }

  #[cfg(feature = "breakpoints")]
  fn check_pc_breakpoints(&mut self, addr: u16) -> Res<()> {
    if self.pc_breakpoints[addr as usize] {
      let instr = self.mmu.rb(addr);
      Err(YargeError::PcBreakpoint { instr, addr })
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
    self.mmu.ppu.tick(&mut self.mmu.iif);
  }

  fn disable_ime(&mut self) {
    self.ime_pending = false;
    self.ime = false;
  }

  fn enable_ime(&mut self) {
    self.ime_pending = true;
  }

  pub fn set_interrupt(iif: &mut u8, int: Interrupt) {
    *iif = *iif | (1 << int as u8);
  }

  fn dispatch_interrupt(&mut self, int: usize) {
    #[cfg(debug_assertions)] {
      assert!(int < 5, "Invalid interrupt: {int}");
    }
    //Unhalt
    if self.state == CpuState::Halt {
      self.state = CpuState::Running;
    }
    //Call interrupt handler
    self.reg.dec_sp(2);
    self.mmu.ww(self.reg.sp, self.reg.pc);
    self.reg.pc = INT_JMP_VEC[int];
    //flip IF bit and disable IME
    self.mmu.iif &= !(1 << int);
    self.disable_ime();
    //Run for 20 cycles
    for _ in 0..5 { self.cycle(); } 
  }

  fn check_interrupts(&mut self) {
    if self.ime_pending {
      self.ime_pending = false;
      self.ime = true;
    }
    let check = self.mmu.iie & self.mmu.iif;
    if check != 0 {
      if self.ime {
        let int_type = check.trailing_zeros() as usize;
        if int_type < 5 {
          self.dispatch_interrupt(int_type);
        }
      } else if self.state == CpuState::Halt {
        self.state = CpuState::Running;
      }
    } 
  }

  pub fn step(&mut self) -> Res<usize> {
    self.t = 0;
    if self.state == CpuState::Running {
      self.check_interrupts();

      #[cfg(feature = "breakpoints")]
      let pc_value = self.reg.pc;

      let mut op = self.fetch()?;
      if op != 0xCB { 
        cpu_instructions!(self, op);
      } else {
        op = self.fetch()?;
        cpu_instructions_cb!(self, op);
      }

      #[cfg(feature = "breakpoints")] {
        self.check_pc_breakpoints(pc_value)?;
      }
    } else {
      self.cycle();
    }
    #[cfg(debug_assertions)]
    assert!(self.t >= 4);
    Ok(self.t)
  }
}
