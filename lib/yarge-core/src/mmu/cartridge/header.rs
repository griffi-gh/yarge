use arrayvec::ArrayString;
use parse_display_derive::Display;

#[derive(Clone, Copy, Default, Debug, Display)]
#[display("Name: {name}\nMBC type: {mbc_type}\nROM size: {rom_size} kb\nRAM size: {ram_size} bytes")]
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
        0 => 0,
        1 => 2 * 1024,
        2 => 8 * 1024,
        3 => 32 * 1024,
        4 => 128 * 1024,
        5 => 64 * 1024,
        _ => 0,
      }
    }
  }
}
