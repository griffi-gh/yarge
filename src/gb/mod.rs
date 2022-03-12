pub mod mmu;
pub mod cpu;
use mmu::MMU;
use cpu::CPU;

pub struct Gameboy {
    pub redraw_needed: bool, //TODO move to ppu
    pub mmu: MMU,
    pub cpu: CPU,
}
impl Gameboy {
    pub fn new() -> Self {
        Self{
            redraw_needed: true,
            mmu: MMU::new(),
            cpu: CPU::new(),
        }
    }
    pub fn step(&mut self) {
        self.cpu.step(&mut self.mmu);
    }
}