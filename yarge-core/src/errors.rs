use thiserror::Error;

#[derive(Error, Debug)]
pub enum YargeError {
  #[error("Invalid or unimplemented CPU instruction: 0x{instr:02X} at {addr:#06X}")]
  InvalidInstruction {
    instr: u8,
    addr: u16,
  },

  #[error("Invalid or unimplemented MBC type: {0:#04X}")]
  InvalidMbcType(u8),

  #[error("Invalid ROM size: {0} bytes")]
  InvalidRomSize(usize),

  // #[error("MMU breakpoint hit: {} at {addr:#06X} with value {value:#04X}", if *is_write { "WRITE" } else { "READ" })]
  // MmuBreakpoint {
  //   is_write: bool,
  //   addr: u16,
  //   value: u8,
  // },

  #[error("PC breakpoint hit: instruction {instr:#04X} at {addr:#06X}")]
  PcBreakpoint {
    addr: u16,
    instr: u8,
  },

  #[error("LD B,B breakpoint hit: at {addr:#06X}")]
  LdBreakpoint {
    addr: u16
  },

  #[error("I/O error")]
  Io {
    #[from] source: std::io::Error
  }
}
impl YargeError {
  pub fn is_recoverable(&self) -> bool {
    matches!(*self, Self::PcBreakpoint{..} | Self::LdBreakpoint{..})
  }
}
