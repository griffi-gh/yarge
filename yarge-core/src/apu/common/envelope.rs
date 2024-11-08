#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub enum EnvelopeDirection {
  #[default] 
  Down, 
  Up
}

#[derive(Default)]
pub struct Envelope {
  pub start_volume: u8,
  pub period: u8,
  pub direction: EnvelopeDirection,
  period_timer: u8,
  curent_volume: u8,
}

impl Envelope {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn to_mmio(&self) -> u8 {
    ((self.start_volume   ) << 4) |
    ((self.direction as u8) << 3) |
    self.period
  }

  pub fn set_from_mmio(&mut self, value: u8) {
    self.period = value & 0x7;
    self.direction = match value & (1 << 3) != 0 {
      false => EnvelopeDirection::Down,
      true  => EnvelopeDirection::Up,
    };
    self.start_volume = value >> 4;
  }

  pub fn trigger(&mut self) {
    self.curent_volume = self.start_volume;
    self.period_timer = self.period;
  }
  
  /// Volume in 0-F range
  #[inline(always)]
  pub fn volume(&self) -> u8 {
    self.curent_volume
  }

  /// Volume in 0.0-1.0 range
  #[inline(always)]
  pub fn volume_f32(&self) -> f32 {
    self.curent_volume as f32 / 15.
  }

  pub fn tick(&mut self) {
    if self.period == 0 {
      return
    }
    if self.period_timer > 0 {
      self.period_timer -= 1;
    }
    if self.period_timer == 0 {
      self.period_timer = self.period;
      match self.direction {
        EnvelopeDirection::Down if self.curent_volume > 0 => {
          self.curent_volume -= 1;
        },
        EnvelopeDirection::Up if self.curent_volume < 0xF => {
          self.curent_volume += 1;
        },
        _ => ()
      }
    }
  }
}
