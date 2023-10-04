use super::ApuChannel;
use crate::consts::audio_registers::*;
use crate::apu::envelope::{Envelope, EnvelopeDirection};

mod wave;
use wave::WaveDuty;

pub struct SquareWaveChannel<const HAS_SWEEP: bool> {
  wave_duty: WaveDuty,
  envelope: Envelope,
  ///a.k.a wavelength
  frequency: u16,
  freq_timer: u16,
  length_timer: u16,
  length_timer_enable: bool,
  //dac_enabled: bool,
  channel_enabled: bool,
}

impl<const HAS_SWEEP: bool> SquareWaveChannel<HAS_SWEEP> {
  pub fn new() -> Self {
    //TODO provide sensilble defaults?
    Self {
      envelope: Envelope::default(),
      wave_duty: WaveDuty::new(),
      freq_timer: 8192, //or 0?
      frequency: 0,
      length_timer: 0,
      length_timer_enable: false,
      //dac_enabled: true,
      channel_enabled: false,
    }
  }

  fn reset_freq_timer(&mut self) {
    self.freq_timer = (2048 - self.frequency) * 4;
  }

  fn trigger(&mut self) {
    self.reset_freq_timer();
    self.channel_enabled = true;
    self.envelope.trigger();
    //XXX: Should this ALWAYS set to 64?
    //self.length_timer = 64;
    if self.length_timer == 0 {
      self.length_timer = 64;
    }
  }
}

impl<const HAS_SWEEP: bool> ApuChannel for SquareWaveChannel<HAS_SWEEP> {
  fn tick_length(&mut self) {
    if !self.channel_enabled { return }
    
    if self.length_timer_enable && self.length_timer > 0 {
      self.length_timer -= 1;
      if self.length_timer == 0 {
        self.channel_enabled = false;
      }
    }
  }

  fn tick_envelope(&mut self) {
    if !self.channel_enabled { return }

    self.envelope.tick()
  }
  
  fn tick(&mut self) {
    if !self.channel_enabled { return }

    if self.freq_timer > 0 {
      self.freq_timer -= 1;
      if self.freq_timer == 0 {
        self.reset_freq_timer();
        self.wave_duty.tick();
        //self.channel_enabled = false;
      }
    }
  }

  fn amplitude(&self) -> f32 {
    if !self.channel_enabled {
      return 0.
    }
    let data = self.wave_duty.get_data();
    //0 => -1.f, 1 => 1.f
    ((data << 1) as i8 - 1) as f32 * self.envelope.volume_f32()
  }
  
  fn read_register(&self, reg: u8) -> u8 {
    //TODO
    0
  }

  fn write_register(&mut self, reg: u8, value: u8) {
    match reg {
      0 if HAS_SWEEP => {
        //TODO
      },
      1 => {
        // 0bAABBBBBB;
        //   I L- freq timer
        //   L- pat type
        self.wave_duty.set_pattern_type((value >> 6) as usize);
        self.length_timer = 64 - (value & 0x3f) as u16;
        //self.channel_enabled = true;
      },
      2 => {
        self.envelope.period = value & 0x7;
        self.envelope.direction = match value & (1 << 3) != 0 {
          false => EnvelopeDirection::Down,
          true  => EnvelopeDirection::Up,
        };
        self.envelope.start_volume = value >> 4;
        // if self.envelope.start_volume == 0 && self.envelope.direction == EnvelopeDirection::Down {
        //   self.channel_enabled = false;
        // }
      },
      3 => {
        self.frequency = (self.frequency & 0x700) | value as u16;
        //XXX: this *may* be correct?
        //self.reset_freq_timer();
      },
      4 => {
        self.frequency = (self.frequency & 0xff) | ((value as u16 & 0b111) << 8);
        //XXX: this *may* be correct?
        //self.reset_freq_timer();
        self.length_timer_enable = value & (1 << 6) != 0;

        if value & 0x80 != 0 {
          //Channel trigerred
          self.trigger();
        }
      },
      _ => ()
    }
  }
}
