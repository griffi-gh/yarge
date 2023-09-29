use crate::consts::AUDIO_BUFFER_SIZE;

#[derive(Clone)]
pub struct AudioBuffer {
  buffer: Box<[f32; AUDIO_BUFFER_SIZE]>,
  head: usize
}

impl AudioBuffer {
  pub fn new() -> Self {
    Self {
      buffer: Box::new([0.; AUDIO_BUFFER_SIZE]),
      head: 0
    }
  }

  pub fn push(&mut self, l_sample: f32, r_sample: f32) {
    assert!(!self.is_full(), "AudioBuffer is full");
    self.buffer[self.head] = l_sample;
    self.buffer[self.head + 1] = r_sample;
    self.head += 2;
  }
  
  pub fn reset(&mut self) {
    self.head = 0;
  }

  pub fn len(&self) -> usize {
    self.head
  }

  pub fn is_full(&self) -> bool {
    self.head >= AUDIO_BUFFER_SIZE
  }
  pub fn get_buffer(&self) -> &[f32; AUDIO_BUFFER_SIZE] {
    self.buffer.as_ref()
  }
}

impl Default for AudioBuffer {
  fn default() -> Self { Self::new() }
}
