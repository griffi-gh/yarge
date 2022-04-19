use std::fmt;
use arrayvec::ArrayString;

#[derive(Clone, Copy, Default, Debug)]
pub struct RomHeader {
  pub name: ArrayString<16>,
  pub mbc_type: u8,
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
      }
    }
  }
}
impl fmt::Display for RomHeader {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let Self {mbc_type, name} = *self;
    write!(formatter, "Name: {name}\nMBC Type: {mbc_type:#04X}")
  }
}
