use seq_macro::seq;

#[derive(Default, Clone, Copy)]
pub struct Terminal {
  pub vin: bool,
  pub volume: u8,
  pub enabled_channels: (bool, bool, bool, bool),
}
impl Terminal {
  pub fn mix_outputs(&self, channels: (f32, f32, f32, f32)) -> f32 {
    let volume = self.volume as f32 / 7.0;
    let mut amplitude = 0.;
    seq!(N in 0..4 {
      if self.enabled_channels.N {
        amplitude += channels.N;
      }
    });
    volume * amplitude / 4.0
  }
}
