#[derive(Default, Clone, Copy)]
pub struct Terminal {
  pub vin: bool,
  pub volume: u8,
  pub enabled_channels: (bool, bool, bool, bool),
}
impl Terminal {
  //This is ridicuosly over-optimized but this greatly improves the generated assembly
  pub fn mix_outputs(&self, channels: (f32, f32, f32, f32)) -> f32 {
    // volume = self.volume / 7.0
    let volume = {
      const VOLUME_LUT: [f32; 8] = [
        0.,
        1. / 7.,
        2. / 7.,
        3. / 7.,
        4. / 7.,
        5. / 7.,
        6. / 7.,
        1.,
      ];
      VOLUME_LUT[(self.volume & 7) as usize]
    };
    let amplitude = {
      f32::from_bits(channels.0.to_bits() * (self.enabled_channels.0 as u32)) +
      f32::from_bits(channels.1.to_bits() * (self.enabled_channels.1 as u32)) +
      f32::from_bits(channels.2.to_bits() * (self.enabled_channels.2 as u32)) +
      f32::from_bits(channels.3.to_bits() * (self.enabled_channels.3 as u32))
    };
    volume * amplitude * 0.25
  }
}
