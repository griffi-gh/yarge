use thiserror::Error;
use crate::consts::MBC_TYPE_NAMES;

#[derive(Error, Debug)]
pub enum YargeError {
  #[error("Invalid or unimplemented CPU instruction: 0x{}{instr:02X} at {addr:#06X}", if *is_cb { "CB" } else { "" })]
  InvalidInstruction {
    is_cb: bool,
    instr: u8,
    addr: u16,
  },

  #[error("Invalid or unimplemented MBC type: {0:#04X} ({})", MBC_TYPE_NAMES.get(.0).unwrap_or(&"INVALID") )]
  InvalidMbcType(u8),

  #[error("Invalid ROM size: {0} bytes")]
  InvalidRomSize(usize),

  #[error("MMU breakpoint hit: {} at {addr:#06X} with value {value:#04X}", if *is_write { "WRITE" } else { "READ" })]
  MmuBreakpoint {
    is_write: bool,
    addr: u16,
    value: u8,
  },

  #[error("PC breakpoint hit: instruction {instr:#04X} at {addr:#06X}")]
  PcBreakpoint {
    addr: u16,
    instr: u8,
  },

  #[error("I/O error")]
  Io {
    #[from] source: std::io::Error
  }
}
impl YargeError {
  pub fn is_recoverable(&self) -> bool {
    match *self {
      Self::PcBreakpoint{..} => true,
      //Self::MmuBreakpoint{..} => true,
      _ => false
    }
  }
}
