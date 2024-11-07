use crate::consts::{AUDIO_CYCLES_PER_SAMPLE, audio_registers::*};
use seq_macro::seq;

mod channels;
mod audio_buffer;
mod audio_device;
mod terminal;
mod common;

use channels::{
  ApuChannel,
  square::SquareWaveChannel,
  noise::NoiseChannel
};
use audio_buffer::AudioBuffer;
pub use audio_device::AudioDevice;
use terminal::Terminal;

pub struct Apu {
  enabled: bool,
  pub device: Option<Box<dyn AudioDevice>>,
  buffer: AudioBuffer,
  /// 0 - CH1 - Square wave
  /// 1 - CH2 - Square wave, No sweep
  channels: (
    SquareWaveChannel<true>,
    SquareWaveChannel<false>,
    (),
    NoiseChannel
  ),
  /// 0 - Right/SO1
  /// 1 - Left /SO2
  terminals: (Terminal, Terminal),
  sequencer: u8,
  sample_cycles: usize,
  prev_div: u16,
}

impl Apu {
  pub fn new() -> Self {
    Self {
      enabled: false,
      device: None,
      buffer: AudioBuffer::new(),
      channels: (
        SquareWaveChannel::new(),
        SquareWaveChannel::new(),
        (),
        NoiseChannel::new()
      ),
      terminals: (Terminal::new(), Terminal::new()),
      sequencer: 0,
      sample_cycles: 0,
      prev_div: 0,
    }
  }

  // > The frame sequencer clocks are derived from the DIV timer. 
  // > In Normal Speed Mode, falling edges of bit 5 step the FS 
  // > while in CGB Double Speed Mode, bit 6 is used instead. 
  // > Here bits 5 and 6 refer to the bits of the upper byte of DIV 
  // > (internally DIV is 16 bit but only the upper 8 bits are mapped to memory).
  // https://nightshade256.github.io/2021/03/27/gb-sound-emulation.html

  fn update_div_falling_edge(&mut self, div: u16) -> bool {
    //GBC: use bit 13 in double speed mode
    //XXX: bit 12? not 13? in sp
    const BIT: u16 = 1 << 12;
    let is_falling_edge = (div & BIT == 0) && (self.prev_div & BIT != 0);
    self.prev_div = div;
    is_falling_edge
  }

  fn tick_all(&mut self) {
    self.channels.0.tick();
    self.channels.1.tick();
    self.channels.3.tick();
  }

  fn tick_length_all(&mut self) {
    self.channels.0.tick_length();
    self.channels.1.tick_length();
    self.channels.3.tick_length();
  }

  fn tick_sweep_all(&mut self) {
    self.channels.0.tick_sweep();
  }

  fn tick_envelope_all(&mut self) {
    self.channels.0.tick_envelope();
    self.channels.1.tick_envelope();
    self.channels.3.tick_envelope();
  }

  fn tick_sequencer(&mut self) {
    //XXX: should sequencer be incremented before of after the match block?\
    self.sequencer = (self.sequencer + 1) & 7;
    match self.sequencer {
      0 | 4 => { // Length only
        self.tick_length_all();
      },
      2 | 6 => { // Length and sweep
        self.tick_length_all();
        self.tick_sweep_all();
      },
      7 => { //Envelope only
        self.tick_envelope_all();
      },
      _ => ()
    }
  }

  pub fn tick(&mut self, div: u16) {
    let is_div_falling_edge = self.update_div_falling_edge(div);

    if !self.enabled { return }

    for _ in 0..4 {
      self.tick_all();
    }

    if is_div_falling_edge {
      self.tick_sequencer();
    }

    self.sample_cycles += 4;
    if self.sample_cycles >= AUDIO_CYCLES_PER_SAMPLE {
      self.sample_cycles -= AUDIO_CYCLES_PER_SAMPLE;
      let amplitudes = (
        self.channels.0.amplitude(),
        self.channels.1.amplitude(),
        0.,
        self.channels.3.amplitude(),
      );
      let samples = (
        self.terminals.0.mix_outputs(amplitudes),
        self.terminals.1.mix_outputs(amplitudes),
      );
      self.buffer.push(samples.1, samples.0);
      //self.buffer.push(self.channel1.amplitude(), self.channel2.amplitude());
      if self.buffer.is_full() {
        if let Some(device) = self.device.as_mut() {
          device.queue_samples(self.buffer.get_buffer());
        }
        self.buffer.reset();
      }
    }
  }

  fn check_write_access(&self, addr: u16) -> bool {
    self.enabled ||
    [R_NR52, R_NR11, R_NR21, R_NR31, R_NR41].contains(&addr) || //GBC: THIS IS NOT THE CASE ON GBC
    (0xff30..=0xff3f).contains(&addr) // Wave pattern ram
  }

  pub fn read(&self, addr: u16) -> u8 {
    match addr {
      //TODO R_NRXX
      R_NR50 => {
        self.terminals.0.volume
        | (self.terminals.1.volume << 4)
      }
      R_NR51 => {
        #[allow(clippy::identity_op, clippy::erasing_op)] {
          seq!(N in 0..4 {
            0
            #(| ((self.terminals.0.enabled_channels.N as u8) << N))*
            #(| ((self.terminals.1.enabled_channels.N as u8) << (N + 4)))*
          })
        }
      },
      R_NR52 => {
        (self.enabled as u8) << 7
        | (self.channels.0.is_enabled() as u8)
        | (self.channels.1.is_enabled() as u8) << 1
        | (self.channels.3.is_enabled() as u8) << 3
      },
      _ => 0
    }
  }

  pub fn write(&mut self, addr: u16, value: u8, blocking: bool) {
    //If the APU is disabled most registers are R/O
    if blocking && !self.check_write_access(addr) { return }
    match addr {
      R_NR10 => self.channels.0.write_register(0, value),
      R_NR11 => self.channels.0.write_register(1, value),
      R_NR12 => self.channels.0.write_register(2, value),
      R_NR13 => self.channels.0.write_register(3, value),
      R_NR14 => self.channels.0.write_register(4, value),
      R_NR21 => self.channels.1.write_register(1, value),
      R_NR22 => self.channels.1.write_register(2, value),
      R_NR23 => self.channels.1.write_register(3, value),
      R_NR24 => self.channels.1.write_register(4, value),
      R_NR41 => self.channels.3.write_register(1, value),
      R_NR42 => self.channels.3.write_register(2, value),
      R_NR43 => self.channels.3.write_register(3, value),
      R_NR44 => self.channels.3.write_register(4, value),
      R_NR50 => {
        self.terminals.0.volume = value & 0x07;
        self.terminals.1.volume = (value >> 4) & 0x07;
      },
      R_NR51 => {
        //these were supposed to be used for this right?
        //haven't touched this codebase for a *while*
        #[allow(clippy::identity_op, clippy::erasing_op)] {
          seq!(S in 0..=1 {
            seq!(N in 0..4 {
              self.terminals.S.enabled_channels.N = (value & (1 << ((S * 4) + N))) != 0;
            });
          });
        }
        //println!("TERMINALS: {:?}", self.terminals);
      }
      R_NR52 => {
        self.enabled = (value & 0x80) != 0;
        //TODO when/if disabled, clear registers
      },
      _ => ()
    }
  }
}

impl Default for Apu {
  fn default() -> Self { Self::new() }
}
