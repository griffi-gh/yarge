use crate::consts::AUDIO_BUFFER_SIZE;

pub trait AudioDevice {
  fn queue_samples(&mut self, buffer: &[f32; AUDIO_BUFFER_SIZE]);
}

// trait QueueIfPresent {
//   fn queue_if_present(&mut self, buffer: &[f32; AUDIO_BUFFER_SIZE]);
// }
// impl QueueIfPresent for Option<Box<dyn AudioDevice>> {
//   fn queue_if_present(&mut self, buffer: &[f32; AUDIO_BUFFER_SIZE]) {
//     if let Some(device) = self {
//       device.queue_samples(buffer);
//     }
//   }
// }
