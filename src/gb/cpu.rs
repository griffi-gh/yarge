mod reg;
use reg::Registers;
use super::mmu::MMU;

pub struct CPU {
    pub reg: Registers,
}
impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
        }
    }
    pub fn step(&mut self, mmu: &mut MMU) {
        let mut op = mmu.read(self.reg.pc());
        let cb = op == 0xCB;
        if cb { op = mmu.read(self.reg.pc()); }
    }
}
