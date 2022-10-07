use crate::consts::AUDIO_BUFFER_SIZE;

pub trait AudioDevice {
    
}

pub struct Apu {
  pub buffer: Box<[f32; AUDIO_BUFFER_SIZE]>
}
impl Apu {
  pub fn new() -> Self {
    Self {
      buffer: Box::new([0.; AUDIO_BUFFER_SIZE])
    }
  }
}
