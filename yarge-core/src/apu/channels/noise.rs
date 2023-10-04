use super::ApuChannel;
use crate::apu::common::{envelope::Envelope, length::LengthTimer};

pub struct NoiseChannel {
  envelope: Envelope,
  length: LengthTimer,
  shift: u8,
  width_short: bool,
  divider: u8,
  freq_timer: u16,
  lfsr: u16,
  channel_enabled: bool,
}

impl NoiseChannel {
  pub fn new() -> Self {
    Self {
      envelope: Envelope::new(),
      length: LengthTimer::new(),
      shift: 0,
      width_short: false,
      divider: 0,
      freq_timer: 0,
      lfsr: 0x1,
      channel_enabled: false,
    }
  }

  pub fn reset_freq_timer(&mut self) {
    let div_value = if self.divider > 0  { self.divider << 4 } else { 8 };
    self.freq_timer = (div_value as u16) << self.shift;
  }

  pub fn trigger(&mut self) {
    self.channel_enabled = true;
    self.reset_freq_timer();
    self.envelope.trigger();
    self.length.trigger();
  }
}

impl ApuChannel for NoiseChannel {
  fn tick_envelope(&mut self) {
    if !self.channel_enabled {
      return
    }
    self.envelope.tick();
  }

  fn tick_length(&mut self) {
    if !self.channel_enabled {
      return
    }
    if self.length.tick() {
      self.channel_enabled = false;
    }
  }

  fn tick(&mut self) {
    if !self.channel_enabled {
      return
    }
    if self.freq_timer > 0 {
      self.freq_timer -= 1;
    }
    if self.freq_timer == 0 {
      self.reset_freq_timer();
      //compute xor of two least significant bits
      let xor_result = (self.lfsr & 0b01) ^ ((self.lfsr & 0b10) >> 1);

      //rotate in xor_result from the left
      self.lfsr = (self.lfsr >> 1) | (xor_result << 14);
      if self.width_short {
        //set bit 6 to xor_result
        self.lfsr = (self.lfsr & !(1 << 6)) | (xor_result << 6);
      }
    }
  }

  fn amplitude(&self) -> f32 {
    if !self.channel_enabled { return 0. }
    let wf = (self.lfsr & 1 == 0) as u8;
    ((wf << 1) as i32 - 1) as f32 * self.envelope.volume_f32()
  }

  fn read_register(&self, _reg: u8) -> u8 {
    //TODO
    0
  }

  fn write_register(&mut self, reg: u8, value: u8) {
    match reg {
      1 => {
        self.length.set_from_inv(value);
      }
      2 => {
        self.envelope.set_from_register(value);
      },
      3 => {
        self.divider = value & 0b111;
        self.width_short = value & 1 << 3 != 0;
        self.shift = value >> 4;
      },
      4 => {
        self.length.enable = value & 1 << 6 != 0;
        if value & 0x80 != 0 {
          self.trigger();
        }
      },
      _ => (),
    }
  }

  fn is_enabled(&self) -> bool {
    self.channel_enabled
  }
}

impl Default for NoiseChannel {
  fn default() -> Self {
    Self::new()
  }
}
