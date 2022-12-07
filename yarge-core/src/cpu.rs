mod reg;
mod instructions;
use instructions::{cpu_instructions, cpu_instructions_cb};
pub use reg::Registers;
use crate::{Mmu, Res, consts::INT_JMP_VEC};

#[derive(Clone, Copy, PartialEq, Eq)]
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
  //Serial  = 3,
  Joypad  = 4,
}

pub struct Cpu {
  pub reg: Registers,
  pub mmu: Mmu,
  pub state: CpuState,
  ime_pending: bool,
  ime: bool,
  t: usize,
  #[cfg(feature = "dbg-breakpoints")]
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
      #[cfg(feature = "dbg-breakpoints")]
      pc_breakpoints: Box::new([false; 0x10000]),
    }
  }

  #[cfg(feature = "dbg-breakpoints")]
  fn check_pc_breakpoints(&mut self, addr: u16) -> Res<()> {
    use crate::YargeError;
    if self.pc_breakpoints[addr as usize] {
      let instr = self.mmu.rb(addr, true);
      Err(YargeError::PcBreakpoint { instr, addr })
    } else {
      Ok(())
    }
  }

  fn rb(&mut self, addr: u16) -> u8 {
    self.cycle();
    self.mmu.rb(addr, true)
  }
  fn wb(&mut self, addr: u16, value: u8) {
    self.cycle();
    self.mmu.wb(addr, value, true);
  }

  fn rw(&mut self, addr: u16) -> u16 {
    self.rb(addr) as u16 | 
    ((self.rb(addr.wrapping_add(1)) as u16) << 8)
  }
  fn ww(&mut self, addr: u16, value: u16) {
    self.wb(addr, (value & 0xFF) as u8);
    self.wb(addr.wrapping_add(1), (value >> 8) as u8);
  }

  fn fetch(&mut self) -> u8 { 
    let op = self.rb(self.reg.pc);
    self.reg.inc_pc(1);
    op
  }
  fn fetch_word(&mut self) -> u16 {
    let op = self.rw(self.reg.pc);
    self.reg.inc_pc(2);
    op
  }

  fn push(&mut self, value: u16) {
    self.reg.dec_sp(2);
    self.ww(self.reg.sp, value);
  }
  fn pop(&mut self) -> u16 {
    let value = self.rw(self.reg.sp);
    self.reg.inc_sp(2);
    value
  }

  fn cycle(&mut self) {
    self.t += 4;
    self.mmu.tick_components();
  }

  fn disable_ime(&mut self) {
    self.ime_pending = false;
    self.ime = false;
  }

  fn enable_ime(&mut self) {
    self.ime_pending = true;
  }

  pub fn set_interrupt(iif: &mut u8, int: Interrupt) {
    *iif |= 1 << int as u8;
  }

  fn dispatch_interrupt(&mut self, int: usize) {
    //Check if interrupt is valid
    debug_assert!(int < 5, "Invalid interrupt: {int}");
    //Call interrupt handler
    self.reg.dec_sp(2);
    self.mmu.ww(self.reg.sp, self.reg.pc, true);
    self.reg.pc = INT_JMP_VEC[int];
    //flip IF bit and disable IME
    self.mmu.iif &= !(1 << int);
    self.disable_ime();
    //Run for 20 cycles
    //TODO spread out cycles???
    for _ in 0..5 { self.cycle(); } 
  }

  fn update_ime(&mut self) {
    if self.ime_pending {
      self.ime = true;
      self.ime_pending = false;
    }
  }
  
  fn check_interrupts(&mut self) {
    let check = self.mmu.iie & self.mmu.iif;
    if check != 0 {
      if self.state == CpuState::Halt {
        self.state = CpuState::Running;
      }
      if self.ime {
        let int_type = check.trailing_zeros() as usize;
        if int_type < 5 {
          self.dispatch_interrupt(int_type);
        }
      }
    }
  }

  pub fn step(&mut self) -> Res<usize> {
    self.t = 0;
    //Check for interrupts
    self.update_ime();
    self.check_interrupts();
    //If isn't running, run for 4 cycles and exit
    if self.state != CpuState::Running {
      self.cycle();
      return Ok(self.t);
    }
    //Remember the PC value for breakpoints
    #[cfg(feature = "dbg-breakpoints")]
    let pc_value = self.reg.pc;
    //Fetch and execute
    let mut op = self.fetch();
    if op != 0xCB { 
      cpu_instructions(self, op)?;
    } else {
      op = self.fetch();
      cpu_instructions_cb(self, op)?;
    }
    //Check for breakpoints
    #[cfg(feature = "dbg-breakpoints")] {
      self.check_pc_breakpoints(pc_value)?;
    }
    //Panic if instruction took less then 4 cycles
    debug_assert!(self.t >= 4);
    Ok(self.t)
  }
}
