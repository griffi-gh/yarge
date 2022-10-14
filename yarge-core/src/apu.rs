mod channels;
mod audio_buffer;
mod audio_device;
mod frame_sequencer;
use audio_buffer::AudioBuffer;
pub use audio_device::AudioDevice;
use channels::square::{SquareWaveChannel, SquareWaveChannelType};

pub struct Apu {
  pub device: Option<Box<dyn AudioDevice>>,
  enabled: bool,
  buffer: AudioBuffer,
  channel1: SquareWaveChannel,
  channel2: SquareWaveChannel,
}
impl Apu {
  pub fn new() -> Self {
    Self {
      device: None,
      enabled: false,
      buffer: AudioBuffer::new(),
      channel1: SquareWaveChannel::new(SquareWaveChannelType::Channel1),
      channel2: SquareWaveChannel::new(SquareWaveChannelType::Channel2),
    }
  }
  pub fn tick(&mut self) {
    
  }
}
impl Default for Apu {
  fn default() -> Self { Self::new() }
}
