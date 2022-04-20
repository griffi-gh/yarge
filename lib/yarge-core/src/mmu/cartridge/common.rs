pub fn rom_addr(addr: u16, bank: u8) -> usize {
  (bank as usize * 0x4000) + (addr as usize - 0x4000)
}
pub fn eram_addr(addr: u16, bank: u8) -> usize {
  (addr as usize - 0xA000) + (bank as usize * 0x2000)
}
