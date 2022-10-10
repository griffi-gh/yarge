use super::gb::{AudioDevice, consts::AUDIO_BUFFER_SIZE};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

struct TestAudioDevice;
impl AudioDevice for TestAudioDevice {
  fn queue_samples(&mut self, _: &[f32; AUDIO_BUFFER_SIZE]) {}
}

//WIP
pub fn init() {
  let host = cpal::default_host();
  let device = host.default_output_device().expect("no output device available");
  let mut supported_configs_range = device.supported_output_configs()
    .expect("Error while querying configs");
  let supported_config = supported_configs_range.next()
    .expect("No supported audio config")
    .with_max_sample_rate();
    //.with_sample_rate(SampleRate(AUDIO_SAMPLE_RATE as u32)); 
  let stream = device.build_output_stream(
    &supported_config.config(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
      for sample in data.iter_mut() {
        *sample = fastrand::f32();
      }
    },
    move |err| Err(err).expect("stream error")
  ).expect("Failed to the create audio stream");
  stream.play().expect("Failed to resume the audio stream");
  //keep playing
  let _ = Box::leak(Box::new(stream));
}
