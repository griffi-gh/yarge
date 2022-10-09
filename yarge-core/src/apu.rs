mod audio_buffer;
mod audio_device;
use audio_buffer::AudioBuffer;
pub use audio_device::AudioDevice;

pub struct Apu {
  enabled: bool,
  buffer: AudioBuffer,
  pub device: Option<Box<dyn AudioDevice>>
}
impl Apu {
  pub fn new() -> Self {
    Self {
      enabled: false,
      buffer: AudioBuffer::new(),
      device: None
    }
  }
}
impl Default for Apu {
  fn default() -> Self { Self::new() }
}
