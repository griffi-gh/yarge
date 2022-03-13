mod reg;
mod instructions;
use instructions::{
    cpu_instructions,
    cpu_instructions_cb
};
pub use reg::Registers;
use super::MMU;

pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(),
        }
    }

    #[inline]
    fn fetch(&mut self) -> u8 { 
        let op = self.mmu.rb(self.reg.pc);
        self.reg.inc_pc(1);
        return op
    }

    //TEST push, pop

    #[inline]
    fn push(&mut self, value: u16) {
        self.mmu.ww(self.reg.dec_sp(2), value);
    }
    #[inline]
    fn pop(&mut self) -> u16 {
        let value = self.mmu.rw(self.reg.sp);
        self.reg.inc_sp(2);
        return value;
    }

    pub fn step(&mut self) {
        let mut op = self.fetch();
        if op != 0xCB { 
            cpu_instructions!(op);
        } else {
            op = self.fetch();
            cpu_instructions_cb!(op);
        }
    }
}
