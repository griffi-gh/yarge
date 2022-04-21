use super::super::RomHeader;
use crate::{Res, YargeError};

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
pub fn load_rom_static<const SIZE: usize>(arr: &mut [u8; SIZE], source: &[u8]) -> Res<()> {
  if source.len() != SIZE {
    return Err(YargeError::InvalidRomSize(source.len()));
  }
  arr.copy_from_slice(source);
  Ok(())
}
pub fn load_rom_vec(vec: &mut Vec<u8>, source: &[u8]) -> Res<()> {
  //TODO check if souce len is valid (size >= 0x8000 && size is multiple of 0x8000)
  vec.clear();
  vec.extend_from_slice(source);
  vec.shrink_to_fit();
  Ok(())
}
