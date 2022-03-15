#![forbid(unsafe_code)]

mod mmu;
mod cpu;
pub use mmu::MMU;
pub use cpu::CPU;

use std::{thread, sync::{Arc, Mutex}};

#[cfg(feature = "logging")]
const LOG_PATH: &str = "./gameboy.log";

const _VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
const _NAME: Option<&str> = option_env!("CARGO_PKG_NAME");

///Gameboy emulator
pub struct Gameboy {
    pub cpu: CPU,
    #[cfg(feature = "logging")]
    file: Option<std::fs::File>,
}
impl Gameboy {
    pub fn new() -> Self {
        Self{
            cpu: CPU::new(),
            #[cfg(feature = "logging")]
            file: None,
        }
    }

    fn _init(&mut self) {
        #[cfg(feature = "logging")] {
            use std::{fs, io::Write};
            let mut file = fs::File::create(LOG_PATH).unwrap();
            file.write_all(b"").unwrap();
            drop(file);
            self.file = Some(fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(LOG_PATH)
                .unwrap());
            write!(
                self.file.as_mut().unwrap(),
                "{} {} log\n",
                _NAME.unwrap_or("<name>"),
                _VERSION.unwrap_or("<version>")
            ).unwrap();
        }
    }
    pub fn init(mut self) -> Self { (&mut self)._init(); self }

    #[cfg(feature = "logging")]
    fn log_step(&mut self) {
        use std::io::Write;
        if self.file.is_none() {
            return;
        }
        let r = &self.cpu.reg;
        let m = &self.cpu.mmu;
        write!(
            self.file.as_mut().unwrap(),
            "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})\n", 
            r.a(), r.f(), r.b(), r.c(), r.d(), r.e(), r.h(), r.l(), r.sp, r.pc,
            m.rb(r.pc), m.rb(r.pc + 1), m.rb(r.pc + 2), m.rb(r.pc + 3)
        ).unwrap();
    }

    pub fn step(&mut self) {
        #[cfg(feature = "logging")] self.log_step();
        let _t = self.cpu.step();
        //TODO Tick other components
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