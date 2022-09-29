use crunchy::unroll;

pub fn spr_line(tile_data: (u8, u8)) -> [u8; 8] {
  let mut colors = [0; 8];
  unroll!{
    for i in 0..8 {
      let mask: u8 = 1 << (7 - i);
      let (l_bit, h_bit) = (
        ((tile_data.0 & mask) != 0) as u8,
        ((tile_data.1 & mask) != 0) as u8
      );
      colors[i] = ((h_bit) << 1) | l_bit;
    }
  }
  colors
}
