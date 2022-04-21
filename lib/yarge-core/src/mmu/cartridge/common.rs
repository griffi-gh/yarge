use super::super::RomHeader;

pub fn rom_addr(addr: u16, bank: u8) -> usize {
  (bank as usize * 0x4000) + (addr as usize - 0x4000)
}
pub fn eram_addr(addr: u16, bank: u8) -> usize {
  (addr as usize - 0xA000) + (bank as usize * 0x2000)
}
pub fn rom_bank_mask(header: &RomHeader) -> u8 {
  ((header.rom_size >> 4) - 1) as u8
}
pub fn eram_bank_mask(header: &RomHeader) -> u8 {
  ((header.ram_size as f32 / 8192.).ceil() as usize).checked_sub(1).unwrap_or(0) as u8
}
