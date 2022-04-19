use super::CartridgeImpl;
use crate::consts::DEFAULT_HEADER;

pub struct MockCartridge;
impl CartridgeImpl for MockCartridge {
  fn name(&self) -> &str { "N/A" }
  fn read_rom(&self, addr: u16) -> u8 {
    let addr = addr as usize - 0x100;
    *DEFAULT_HEADER.get(addr).unwrap_or(&0)
  }
}
