use crate::{apu::{ApuChannel, wave::WaveDuty}, consts::audio_registers::*};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SquareWaveChannelType {
  Channel1,
  Channel2
}

pub struct SquareWaveChannel {
  channel_type: SquareWaveChannelType,
  wave_duty: WaveDuty,
  ///a.k.a wavelength
  frequency: u16,
  freq_timer: u16,
  length_timer: u16,
  length_timer_enable: bool,
  //dac_enabled: bool,
  channel_enabled: bool,
}

impl SquareWaveChannel {
  pub fn new(channel_type: SquareWaveChannelType) -> Self {
    //TODO provide sensilble defaults?
    Self {
      channel_type,
      wave_duty: WaveDuty::new(),
      freq_timer: 8192, //or 0?
      frequency: 0,
      length_timer: 0,
      length_timer_enable: false,
      //dac_enabled: true,
      channel_enabled: false,
    }
  }
}

impl ApuChannel for SquareWaveChannel {
  fn tick(&mut self) {
    if !self.channel_enabled { return }
    //self.freq_timer -= 1;
    
    if self.length_timer_enable && self.length_timer > 0 {
      self.length_timer -= 1;
      if self.length_timer == 0 {
        self.channel_enabled = false;
      }
    }

    self.freq_timer = self.freq_timer.saturating_sub(1);
    if self.freq_timer == 0 {
      self.freq_timer = (2048 - self.frequency) * 4;
      self.wave_duty.tick();
      self.channel_enabled = false;
    }
  }

  fn amplitude(&self) -> f32 {
    if !self.channel_enabled {
      return 0.
    }
    let data = self.wave_duty.get_data() as i8;
    ((data << 1) - 1) as f32
    // let data = self.wave_duty.get_data() as f32;
    // //idk why /7.5 - 1 part is needed, I stole it from another emu
    // (data / 7.5) - 1.0 
  }
  
  fn read(&self, mmio_addr: u16) -> u8 {
    0
  }

  fn write(&mut self, mmio_addr: u16, value: u8) {
    match mmio_addr { 
      R_NR10 => {
        //TODO
      },
      R_NR11 | R_NR21 => {
        // 0bAABBBBBB;
        //   I L- freq timer
        //   L- pat type
        self.wave_duty.set_pattern_type((value >> 6) as usize);
        self.length_timer = 64 - (value & 0x3f) as u16;
        //self.channel_enabled = true;
      },
      R_NR12 | R_NR22 => {
        //TODO
      },
      R_NR13 | R_NR23 => {
        self.frequency = (self.frequency & 0x700) | value as u16;
      },
      R_NR14 | R_NR24 => {
        if value & 0x80 != 0 {
          //Channel trigerred
          self.channel_enabled = true;
        }
        self.frequency = (self.frequency & 0xff) | ((value as u16 & 0b111) << 8);
        self.length_timer_enable = value & (1 << 6) != 0;
      },
      _ => ()
    }
  }
}
