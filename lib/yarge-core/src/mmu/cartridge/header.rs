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
        let mut str = ArrayString::<16>::new();
        for addr in 0x134..=0x143_usize {
          let byte = rom[addr];
          if byte == 0 {
            break;
          } else {
            str.push(char::from_u32(byte as u32).unwrap());
          }
        }
        str
      }
    }
  }
}
impl fmt::Display for RomHeader {
  fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    let mbc_type = self.mbc_type;
    let name = &self.name[..];
    write!(formatter, "Name: {name}\nMBC Type: {mbc_type:#04X}")
  }
}
