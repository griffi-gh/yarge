mod channels;
mod audio_buffer;
mod audio_device;
mod frame_sequencer;

pub use audio_device::AudioDevice;
use audio_buffer::AudioBuffer;
use channels::square::{SquareWaveChannel, SquareWaveChannelType};
use frame_sequencer::FrameSequencer;

pub struct Apu {
  pub device: Option<Box<dyn AudioDevice>>,
  enabled: bool,
  buffer: AudioBuffer,
  sequencer: FrameSequencer,
  channel1: SquareWaveChannel,
  channel2: SquareWaveChannel,
}
impl Apu {
  pub fn new() -> Self {
    Self {
      device: None,
      enabled: false,
      buffer: AudioBuffer::new(),
      sequencer: FrameSequencer::new(),
      channel1: SquareWaveChannel::new(SquareWaveChannelType::Channel1),
      channel2: SquareWaveChannel::new(SquareWaveChannelType::Channel2),
    }
  }
  pub fn tick(&mut self) {
    if !self.enabled { return }
    self.channel1.tick();
    self.channel2.tick();
    match self.sequencer.tick() {
      _ => todo!()
    }
  }
}
impl Default for Apu {
  fn default() -> Self { Self::new() }
}
