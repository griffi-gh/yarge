mod reg;
mod instructions;
use instructions::*; //XXX
pub use reg::Registers;
use super::MMU;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    t: u32,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(),
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

    fn push(&mut self, value: u16) {
        self.reg.dec_sp(2);
        self.ww(self.reg.sp, value);
    }
    fn pop(&mut self) -> u16 {
        let value = self.rw(self.reg.sp);
        self.reg.inc_sp(2);
        return value;
    }

    #[inline]
    fn rb(&mut self, addr: u16) -> u8 {
        self.t += 4;
        self.mmu.rb(addr)
    }
    #[inline]
    fn wb(&mut self, addr: u16, value: u8) {
        self.t += 4;
        self.mmu.wb(addr, value);
    }

    #[inline]
    fn rw(&mut self, addr: u16) -> u16 {
        self.t += 8;
        self.mmu.rw(addr)
    }
    #[inline]
    fn ww(&mut self, addr: u16, value: u16) {
        self.t += 8;
        self.mmu.ww(addr, value);
    }

    #[inline(always)]
    fn internal(&mut self, cycles: u32) {
        self.t += cycles;
    }

    pub fn step(&mut self) -> u32 {
        self.t = 0;
        let mut op = self.fetch();
        if op != 0xCB { 
            cpu_instructions!(self, op);
        } else {
            op = self.fetch();
            cpu_instructions_cb!(self, op);
        }
        let t = self.t;
        self.t = 0;
        return t;
    }
}
