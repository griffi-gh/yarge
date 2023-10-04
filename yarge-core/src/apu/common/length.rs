#[derive(Default)]
pub struct LengthTimer {
  pub timer: u8,
  pub enable: bool,
}

impl LengthTimer {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set_from_inv(&mut self, value: u8) {
    self.timer = 64 - (value & 0x3f);
  }

  ///Returns true if the channel needs to be disabled
  pub fn tick(&mut self) -> bool {
    if self.enable && self.timer > 0 {
      self.timer -= 1;
      self.timer == 0
    } else {
      false
    }
  }

  pub fn trigger(&mut self) {
    //XXX: Should this ALWAYS set to 64?
    //self.length_timer = 64;
    if self.timer == 0 {
      self.timer = 64;
    }
  }
}
