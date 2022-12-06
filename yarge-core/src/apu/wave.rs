const WAVE_DUTY_PATTERNS: [u8; 4] = [
  0b00000001, // 12.5%
  0b00000011, // 25%
  0b00001111, // 50%
  0b11111100, // 75%
];

pub struct WaveDuty {
  pattern_type: usize,
  pattern: u8,
  position: u32,
}
impl WaveDuty {
  pub fn new() -> Self {
    Self {
      pattern_type: 0,
      pattern: WAVE_DUTY_PATTERNS[0],
      position: 0,
    }
  }
  pub fn tick(&mut self) {
    self.position += 1;
    self.position &= 7;
  }
  pub fn get_data(&self) -> u8 {
    (self.pattern.rotate_left(self.position) & 0x80) >> 7
  }
  pub fn set_pattern_type(&mut self, pattern_type: usize) {
    debug_assert!(pattern_type < 4, "Invalid pattern type {}", pattern_type);
    self.pattern_type = pattern_type;
    self.pattern = WAVE_DUTY_PATTERNS[pattern_type & 0b11];
  }
  pub fn get_pattern_type(&self) -> usize {
    self.pattern_type
  }
}
impl Default for WaveDuty {
  fn default() -> Self { Self::new() }
}
