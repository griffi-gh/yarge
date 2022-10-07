use crate::consts::AUDIO_BUFFER_SIZE;

mod audio_buffer;
use audio_buffer::AudioBuffer;

pub trait AudioDevice {
  fn queue_samples(&mut self, buffer: &[f32; AUDIO_BUFFER_SIZE]);
}

pub struct Apu {
  enabled: bool,
  buffer: AudioBuffer
}
impl Apu {
  pub fn new() -> Self {
    Self {
      enabled: false,
      buffer: AudioBuffer::new()
    }
  }
}
impl Default for Apu {
  fn default() -> Self { Self::new() }
}
