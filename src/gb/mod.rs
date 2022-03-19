#![forbid(unsafe_code)]

mod mmu;
mod cpu;
mod ppu;
pub use mmu::MMU;
pub use cpu::CPU;
pub use ppu::PPU;

use std::{thread, sync::{Arc, Mutex}, error::Error};

#[cfg(feature = "logging-file")]
const LOG_PATH: &str = "./gameboy.log";

pub struct GameboyBuilder {
    gb: Gameboy,
}
type Res<T> = Result<T, Box<dyn Error + 'static>>;
impl GameboyBuilder {
    pub fn new() -> Self {
        Self {
            gb: Gameboy::new(),
        }
    }
    pub fn init(mut self, cond: bool) -> Self {
        if cond {
            (&mut self).gb.init();
        }
        return self;
    }
    pub fn skip_bootrom(mut self, cond: bool) -> Self {
        if cond {
            (&mut self).gb.skip_bootrom();
        }
        return self;
    }
    pub fn load_rom(mut self, data: &[u8]) -> Self {
        (&mut self).gb.load_rom(data);
        return self;
    }
    pub fn load_rom_file(mut self, path: &str) -> Res<Self> {
        match (&mut self).gb.load_rom_file(path) {
            Ok(()) => Ok(self),
            Err(e) => Err(e)
        }
    }
    pub fn build(self) -> Gameboy { self.gb }
}


///Gameboy emulator
pub struct Gameboy {
    pub cpu: CPU,
    #[cfg(feature = "logging-file")]
    log_file: Option<std::fs::File>,
}
#[allow(dead_code)]
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
    pub fn load_rom_file(&mut self, path: &str) -> Res<()> {
        self.cpu.mmu.cart.load_file(path)
    }
    pub fn load_rom(&mut self, data: &[u8]) {
        self.cpu.mmu.cart.load(data);
    }
    pub fn skip_bootrom(&mut self) {
        if self.cpu.mmu.bios_disabled {
            panic!("Attempt to skip bios while not in bootrom");
        }
        let reg = &mut self.cpu.reg;
        reg.pc = 0x0100;
        reg.sp = 0xFFFE;
        reg.set_af(0x01B0);
        reg.set_bc(0x0013);
        reg.set_de(0x00D8);
        reg.set_hl(0x014D);
        self.cpu.mmu.bios_disabled = true;
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