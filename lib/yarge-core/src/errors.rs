use std::{fmt, error::Error};

#[derive(Debug, Clone)]
pub struct InvalidInstrError {
  pub is_cb: bool,
  pub instr: u8,
  pub addr: u16,
}
impl fmt::Display for InvalidInstrError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f, "Invalid CPU instruction 0x{}{:02X} at {:#06X}",
      if self.is_cb { "CB" } else { "" },
      self.instr, self.addr
    )
  }
}
impl Error for InvalidInstrError {}

#[derive(Debug, Clone)]
pub struct RomLoadError {
  pub reason: String
}
impl fmt::Display for RomLoadError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f, "Failed to load ROM file\n{}",
      self.reason
    )
  }
}
impl Error for RomLoadError {}

#[derive(Debug, Clone)]
pub struct InvalidMBCError {
  pub mbc: u8,
}
impl fmt::Display for InvalidMBCError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f, "Invalid MBC type: {:#04X}",
      self.mbc
    )
  }
}
impl Error for InvalidMBCError {}

#[derive(Debug, Clone)]
pub struct BreakpointHitError {
  pub is_pc: bool,
  pub addr: u16,
  pub value: Option<u8>,
}
impl fmt::Display for BreakpointHitError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let Self { addr, value, is_pc } = self;
    let value = value.unwrap_or_default();
    write!(
      f, "{0} Breakpoint hit {addr:#06X} {value}",
      if *is_pc { "PC" } else { "MMU" }
    )
  }
}
impl Error for BreakpointHitError {}
