pub const WAVE_DUTY_PATTERNS: [[u8; 8]; 4] = [
  [0, 0, 0, 0, 0, 0, 0, 1], // 12.5%
  [1, 0, 0, 0, 0, 0, 0, 1], // 25%
  [1, 0, 0, 0, 0, 1, 1, 1], // 50%
  [0, 1, 1, 1, 1, 1, 1, 0], // 75%
];

pub struct WaveDuty {
  pub pattern: [u8; 8],
  position: usize,
}
impl WaveDuty {
  pub fn new() -> Self {
    Self {
      pattern: WAVE_DUTY_PATTERNS[0],
      position: 0,
    }
  }
  pub fn tick(&mut self) {
    self.position += 1;
    self.position &= 0x7;
  }
  pub fn get_data(&self) -> u8 {
    self.pattern[self.position]
  }
}
