#![forbid(unsafe_code)]

mod mmu;
mod cpu;
pub use mmu::MMU;
pub use cpu::CPU;

use std::{thread, sync::{Arc, Mutex}};

#[cfg(feature = "logging-file")]
const LOG_PATH: &str = "./gameboy.log";

///Gameboy emulator
pub struct Gameboy {
    pub cpu: CPU,
    #[cfg(feature = "logging-file")]
    file: Option<std::fs::File>,
}
impl Gameboy {
    pub fn new() -> Self {
        Self{
            cpu: CPU::new(),
            #[cfg(feature = "logging-file")] file: None,
        }
    }

    fn _init(&mut self) {
        #[cfg(feature = "logging-file")] {
            use std::{fs, io::Write};
            let mut file = fs::File::create(LOG_PATH).unwrap();
            file.write_all(b"").unwrap();
            drop(file);
            self.file = Some(fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(LOG_PATH)
                .unwrap());
        }
    }
    pub fn init(mut self) -> Self { (&mut self)._init(); self }

    #[cfg(feature = "logging")]
    fn log_step(&mut self) {
        let r = &self.cpu.reg;
        let m = &self.cpu.mmu;
        let string = format!(
            "A: {:02X} F: {:02X} B: {:02X} C: {:02X} D: {:02X} E: {:02X} H: {:02X} L: {:02X} SP: {:04X} PC: 00:{:04X} ({:02X} {:02X} {:02X} {:02X})", 
            r.a(), r.f(), r.b(), r.c(), r.d(), r.e(), r.h(), r.l(), r.sp, r.pc,
            m.rb(r.pc), m.rb(r.pc + 1), m.rb(r.pc + 2), m.rb(r.pc + 3)
        );
        #[cfg(feature = "logging-file")] {
            if self.file.is_none() {
                panic!("File not inited!");
            } else {
                use std::io::Write;
                write!(self.file.as_mut().unwrap(), "{}\n", string).unwrap();
            }
        }
        #[cfg(feature = "logging-stdout")] {
            println!("{}", string);
        }
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