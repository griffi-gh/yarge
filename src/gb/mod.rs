pub mod mmu;
pub mod cpu;
use mmu::MMU;
use cpu::CPU;

use std::{thread, sync::{Arc, Mutex}};

///Gameboy emulator
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

    pub fn run(gb: &mut Gameboy) {
        loop { gb.step(); }
    }
    pub fn run_thread(gb: &Arc<Mutex<Gameboy>>) -> thread::JoinHandle<()> {   
        let gb = Arc::clone(&*gb);
        thread::spawn(move || {
            loop {
                let mut gb = gb.lock().unwrap();
                gb.step();
                drop(gb);
            }
        })
    }
}