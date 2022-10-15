use seq_macro::seq;

pub fn spr_line(tile_data: (u8, u8)) -> [u8; 8] {
  let mut colors = [0; 8];
  seq!(N in 0..8 {{
    #[allow(clippy::eq_op)]
    #[allow(clippy::identity_op)]
    const MASK: u8 = 1 << (7 - N);
    let (l_bit, h_bit) = (
      ((tile_data.0 & MASK) != 0) as u8,
      ((tile_data.1 & MASK) != 0) as u8
    );
    colors[N] = ((h_bit) << 1) | l_bit;
  }});
  colors
}
