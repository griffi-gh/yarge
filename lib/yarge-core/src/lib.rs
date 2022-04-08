#![forbid(unsafe_code)]
pub(crate) mod mmu;
pub(crate) mod cpu;
pub(crate) mod ppu;
pub(crate) use mmu::MMU;
pub(crate) use cpu::CPU;
pub(crate) use ppu::PPU;
pub mod errors;
pub mod consts;
mod api;
pub use api::*;

use std::error::Error;
use consts::CYCLES_PER_FRAME;

#[cfg(feature = "logging-file")]
use std::fs::File;
#[cfg(feature = "logging-file")]
use consts::LOG_PATH;

pub(crate) type Res<T> = Result<T, Box<dyn Error>>;

///Gameboy emulator
pub struct Gameboy {
  pub running: bool,
  cpu: CPU,
  #[cfg(feature = "logging-file")] 
  log_file: Option<File>,
}
impl Gameboy {
  pub fn new() -> Self {
    Self {
      running: true,
      cpu: CPU::new(),
      #[cfg(feature = "logging-file")]
      log_file: None,
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
      println!(
        "Writing trace log to file: \"{}\"\n\
        Warning! The log files can get as big as a few gigabytes in size!\n\
        Build without the 'logging-file' feature to disable.", 
        LOG_PATH
      )
    }
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

  #[inline] pub fn pause(&mut self) {
    self.running = false;
  }
  #[inline] pub fn resume(&mut self) {
    self.running = true;
  }
  
  pub fn reset(&mut self) {
    self.cpu = CPU::new();
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
        panic!("File not inited! Call <Gameboy>.init() or <GameboyBuilder>.init()");
      } else {
        use std::io::Write;
        write!(self.log_file.as_mut().unwrap(), "{}\n", string).unwrap();
      }
    }
    #[cfg(feature = "logging-stdout")] {
      println!("{}", string);
    }
  }

  pub fn step(&mut self) -> Result<usize, Box<dyn Error>> {
    if !self.running { return Ok(0); }
    #[cfg(feature = "logging")] self.log_step();
    let cycles = self.cpu.step()?;
    Ok(cycles)
  }

  pub fn run_for_frame(&mut self) -> Res<()> {
    if !self.running {
      return Ok(());
    }
    let mut t = 0;
    while t <= CYCLES_PER_FRAME {
      t += self.step()?;
    }
    Ok(())
  }
  pub fn run(&mut self) -> Res<()> {
    loop { self.step()?; }
  }
}
