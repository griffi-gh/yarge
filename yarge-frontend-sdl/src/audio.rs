use yarge_core::{
  AudioDevice as AudioDeviceImpl,
  consts::{AUDIO_BUFFER_SIZE, AUDIO_SAMPLE_RATE},
};
use sdl2::{
  audio::{AudioQueue, AudioSpecDesired},
  Sdl,
};

pub struct AudioDevice {
  queue: AudioQueue<f32>,
}
impl AudioDevice {
  pub fn new(context: &Sdl) -> Result<Self, Box<dyn std::error::Error>> {
    let audio_subsystem = context.audio()?;
    let audio_spec = AudioSpecDesired {
      freq: Some(AUDIO_SAMPLE_RATE as i32),
      samples: Some(AUDIO_BUFFER_SIZE as u16),
      channels: Some(2),
    };
    let queue = audio_subsystem.open_queue(None, &audio_spec)?;
    queue.resume();
    Ok(Self { queue })
  }
}
impl AudioDeviceImpl for AudioDevice {
  fn queue_samples(&mut self, buffer: &[f32; AUDIO_BUFFER_SIZE]) {
    self.queue.queue_audio(buffer).unwrap();
  }
}