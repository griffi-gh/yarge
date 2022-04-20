use std::fmt;
use arrayvec::ArrayString;

#[derive(Clone, Copy, Default, Debug)]
pub struct RomHeader {
  pub name: ArrayString<16>,
  pub mbc_type: u8,
  pub rom_size: usize,
  pub ram_size: usize,
}
impl RomHeader {
  pub fn parse(rom: &[u8]) -> Self {
    Self {
      mbc_type: rom[0x147],
      name: {
        let mut string = ArrayString::<16>::new();
        for addr in 0x134..=0x143_usize {
          let byte = rom[addr];
          if byte == 0 {
            break;
          } else {
            string.push(char::from_u32(byte as u32).unwrap());
          }
        }
        string
      },
      rom_size: 32 << rom[0x148],
      ram_size: match rom[0x149] {
        _ => 0,
        0 => 0,
        1 => 2,
        2 => 8,
        3 => 32,
        4 => 128,
        5 => 64,
      }
    }
  }
}
impl fmt::Display for RomHeader {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let Self {mbc_type, name, rom_size, ram_size} = *self;
    write!(formatter, "Name: {name}\nMBC Type: {mbc_type:#04X}")
  }
}
