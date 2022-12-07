use crate::apu::{ApuChannel, wave::WaveDuty};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SquareWaveChannelType {
  Channel1,
  Channel2
}

pub struct SquareWaveChannel {
  channel_type: SquareWaveChannelType,
  wave_duty: WaveDuty,
  wavelength: usize,
  freq_timer: usize,
  channel_enabled: bool,
}
impl SquareWaveChannel {
  pub fn new(channel_type: SquareWaveChannelType) -> Self {
    //TODO provide sensilble defaults?
    Self {
      channel_type,
      wave_duty: WaveDuty::new(),
      freq_timer: 8192, //or 0?
      wavelength: 0,
      channel_enabled: false,
    }
  }
}
impl ApuChannel for SquareWaveChannel {
  fn tick(&mut self) {
    // tick is called for each M-cycle, so loop 4 times
    // because 1M = 4T
    for _ in 0..4 {
      self.freq_timer -= 1;
      if self.freq_timer == 0 {
        self.freq_timer = 4 * (2048 - self.wavelength);
        self.wave_duty.tick();
      }
    }
  }
  fn amplitude(&self) -> f32 {
    if !self.channel_enabled {
      return 0.
    }
    let data = self.wave_duty.get_data() as f32;
    //idk why /7.5 - 1 part is needed, I stole it from another emu
    (data / 7.5) - 1.0 
  }
}
