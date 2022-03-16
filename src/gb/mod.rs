#![forbid(unsafe_code)]

mod mmu;
mod cpu;
mod ppu;
pub use mmu::MMU;
pub use cpu::CPU;
pub use ppu::PPU;

use std::{thread, sync::{Arc, Mutex}};

#[cfg(feature = "logging-file")]
const LOG_PATH: &str = "./gameboy.log";

pub struct GameboyBuilder {
    gb: Gameboy,
    err: Option<Box<dyn std::error::Error + 'static>>,
}
impl GameboyBuilder {
    pub fn new() -> Self {
        Self {
            gb: Gameboy::new(),
            err: None
        }
    }
    fn check_err(&self) -> bool { self.err.is_some() }
    pub fn init(mut self) -> Self {
        if (&self).check_err() { return self }
        (&mut self).gb.init();
        return self;
    }
    pub fn load_rom(mut self, data: &[u8]) -> Self {
        if (&self).check_err() { return self }
        (&mut self).gb.load_rom(data);
        return self;
    }
    pub fn load_rom_file(mut self, path: &String) -> Self {
        if (&self).check_err() { return self }
        (&mut self).gb.load_rom_file(&*path).unwrap_or_else(|e| {
            self.err = Some(e);
        });
        return self;
    }
    pub fn build(self) -> Result<Gameboy, Box<dyn std::error::Error + 'static>> {
        let gb = self.gb;
        let err = self.err;
        match err {
            Some(err) => Err(err),
            None => Ok(gb)
        }
    }
}


///Gameboy emulator
pub struct Gameboy {
    pub cpu: CPU,
    #[cfg(feature = "logging-file")]
    log_file: Option<std::fs::File>,
}
impl Gameboy {
    pub fn new() -> Self {
        Self{
            cpu: CPU::new(),
            #[cfg(feature = "logging-file")] log_file: None,
        }
    }

    pub fn init(&mut self) {
        #[cfg(feature = "logging-file")] {
            use std::{fs, io::Write};
            let mut file = fs::File::create(LOG_PATH).unwrap();
            file.write_all(b"").unwrap();
            drop(file);
            self.log_file = Some(fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(LOG_PATH)
                .unwrap());
        }
    }
    pub fn load_rom_file(&mut self, path: &String) -> Result<(), Box<dyn std::error::Error + 'static>> {
        self.cpu.mmu.cart.load_file(path)?; Ok(())
    }
    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.mmu.cart.load(data);
    }

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
            if self.log_file.is_none() {
                panic!("File not inited!");
            } else {
                use std::io::Write;
                write!(self.log_file.as_mut().unwrap(), "{}\n", string).unwrap();
            }
        }
        #[cfg(feature = "logging-stdout")] {
            println!("{}", string);
        }
    }

    pub fn step(&mut self) -> u32 {
        #[cfg(feature = "logging")] self.log_step();
        self.cpu.step()
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