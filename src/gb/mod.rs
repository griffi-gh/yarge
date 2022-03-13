#![cfg_attr(
    feature = "no_unsafe",
    forbid(unsafe_code)
)] // Forbid unsafe code if the "no_unsafe" feature is on

mod mmu;
mod cpu;
pub use mmu::MMU;
pub use cpu::CPU;

use std::{thread, sync::{Arc, Mutex}};

///Gameboy emulator
pub struct Gameboy {
    pub cpu: CPU,
}
impl Gameboy {
    pub fn new() -> Self {
        Self{
            cpu: CPU::new(),
        }
    }
    pub fn step(&mut self) {
        self.cpu.step();
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