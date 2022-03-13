mod reg;
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
    pub fn step(&mut self) {
        let mut op = self.fetch();
        if op != 0xCB { 
            match op {
                //TODO 0xCB [OP] instructions
                _ => panic!("Invalid instruction")
            }
        } else {
            op = self.fetch();
            match op {
                //TODO Instructions
                _ => panic!("Invalid instruction")
            }
        }
    }
}
