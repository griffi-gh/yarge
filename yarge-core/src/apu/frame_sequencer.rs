const FRAME_SEQUENCER_MAX_CYCLES: u16 = 8192; // 8192 cycles = 512 hz
const FRAME_SEQUENCER_STEPS: u8 = 8;

#[derive(Default, Clone, Copy)]
pub struct FrameSequencer {
  cycles: u16,
  step: u8,
}
impl FrameSequencer {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn tick(&mut self) -> Option<u8> {
    self.cycles += 1;
    (self.cycles >= FRAME_SEQUENCER_MAX_CYCLES).then(|| {
      self.cycles = 0;
      self.step += 1;
      self.step &= FRAME_SEQUENCER_STEPS;
      self.step
    })
  }
}
