use super::gb::{AudioDevice, consts::AUDIO_BUFFER_SIZE};

struct TestAudioDevice;
impl AudioDevice for TestAudioDevice {
  fn queue_samples(&mut self, _: &[f32; AUDIO_BUFFER_SIZE]) {}
}
