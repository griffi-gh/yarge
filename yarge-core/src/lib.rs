#![forbid(unsafe_code)]

//Components
pub(crate) mod mmu;
pub(crate) mod cpu;
pub(crate) mod ppu;
pub(crate) mod timers;
pub(crate) mod input;
pub(crate) use mmu::Mmu;
pub(crate) use cpu::Cpu;
pub(crate) use ppu::Ppu;
pub(crate) use timers::Timers;
pub(crate) use input::Input;

//Modules
pub mod consts;
mod errors;
mod api;

//Re-exports
pub use input::Key;
pub use cpu::CpuState;
pub use errors::YargeError;
pub use api::*;
pub use consts::VERSION;

//Types
pub(crate) type Res<T> = Result<T, YargeError>;

///Gameboy emulator
pub struct Gameboy {
  pub running: bool,
  cpu: Cpu,
  #[cfg(feature = "logging-file")] 
  log_file: Option<std::fs::File>,
}
impl Gameboy {
  pub fn new() -> Self {
    Self {
      running: true,
      cpu: Cpu::new(),
      #[cfg(feature = "logging-file")]
      log_file: None,
    }
  }

  pub fn init(&mut self) {
    #[cfg(feature = "logging-file")] {
      use std::{fs, io::Write};
      use consts::LOG_PATH;
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
  
  pub fn reset(&mut self) {
    self.cpu = Cpu::new();
  }

  #[cfg(feature = "logging")]
  fn log_step(&mut self) {
    let r = &self.cpu.reg;
    let m = &self.cpu.mmu;
    let string = format!(
      "A: {a:02X} F: {f:02X} B: {b:02X} C: {c:02X} \
      D: {d:02X} E: {e:02X} H: {h:02X} L: {l:02X} \
      SP: {sp:04X} PC: 00:{pc:04X} \
      ({rb0:02X} {rb1:02X} {rb2:02X} {rb3:02X})", 
      a = r.a(), f = r.f(), 
      b = r.b(), c = r.c(), 
      d = r.d(), e = r.e(), 
      h = r.h(), l = r.l(), 
      sp = r.sp, pc = r.pc,
      rb0 = m.rb(r.pc + 0, true),
      rb1 = m.rb(r.pc + 1, true), 
      rb2 = m.rb(r.pc + 2, true),
      rb3 = m.rb(r.pc + 3, true)
    );
    #[cfg(feature = "logging-file")] {
      use std::io::Write;
      assert!(self.log_file.is_some(), "File not inited! Call <Gameboy>.init() or <GameboyBuilder>.init()");
      write!(self.log_file.as_mut().unwrap(), "{}\n", string).unwrap();
    }
    #[cfg(feature = "logging-stdout")] {
      println!("{}", string);
    }
  }

  pub fn ignore_running<R, F: FnMut(&mut Self) -> R>(&mut self, ctx: &mut F) -> R {
    let state = self.running;
    self.resume();
    let ret: R = ctx(self);
    self.running = state;
    return ret;
  }

  pub fn step(&mut self) -> Res<usize> {
    if !self.running { return Ok(0); }
    #[cfg(feature = "logging")] self.log_step();
    let cycles = self.cpu.step()?;
    Ok(cycles)
  }

  pub fn run_for_frame(&mut self) -> Res<()> {
    if !self.running {
      return Ok(());
    }
    use consts::CYCLES_PER_FRAME;
    self.reset_frame_ready();
    let mut cycles: usize = 0;
    while !(self.get_frame_ready() || cycles >= CYCLES_PER_FRAME) {
      cycles += self.step()?;
    }
    Ok(())
  }

  #[inline] pub fn run(&mut self) -> Res<()> {
    loop { self.step()?; }
  }
}