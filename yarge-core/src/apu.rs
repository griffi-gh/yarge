use crate::consts::{AUDIO_CYCLES_PER_SAMPLE, audio_registers::*};
use seq_macro::seq;

mod channels;
mod audio_buffer;
mod audio_device;
mod terminal;
mod traits;
mod envelope;

pub use traits::ApuChannel;
pub use audio_device::AudioDevice;
use audio_buffer::AudioBuffer;
use channels::square::{SquareWaveChannel, SquareWaveChannelType};
use terminal::Terminal;

pub struct Apu {
  pub device: Option<Box<dyn AudioDevice>>,
  enabled: bool,
  buffer: AudioBuffer,
  sequencer: u8,
  channel1: SquareWaveChannel,
  channel2: SquareWaveChannel,
  sample_cycles: usize,
  /// 0 - Right/SO1
  /// 1 - Left /SO2
  terminals: (Terminal, Terminal),
  prev_div: u16,
}

impl Apu {
  pub fn new() -> Self {
    Self {
      device: None,
      enabled: false,
      buffer: AudioBuffer::new(),
      sequencer: 0,
      channel1: SquareWaveChannel::new(SquareWaveChannelType::Channel1),
      channel2: SquareWaveChannel::new(SquareWaveChannelType::Channel2),
      sample_cycles: 0,
      terminals: (Terminal::new(), Terminal::new()),
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
    //GBC: use bit 14 in double speed mode
    let is_falling_edge = ((div >> 13) & 1 == 0) && ((self.prev_div >> 13) & 1 != 0);
    self.prev_div = div;
    is_falling_edge
  }

  fn tick_sequencer(&mut self) {
    //XXX: should sequencer be incremented before of after the match block?\
    self.sequencer = (self.sequencer + 1) & 7;
    match self.sequencer {
      0 | 4 => { // Length only
        self.channel1.tick_length();
        self.channel2.tick_length();
      },
      2 | 6 => { // Length and sweep 
        self.channel1.tick_length();
        self.channel2.tick_length();
        self.channel1.tick_sweep();
      },
      7 => { //Envelope only
        self.channel1.tick_envelope();
        self.channel2.tick_envelope();
      },
      _ => ()
    }   
  }

  pub fn tick(&mut self, div: u16) {
    let is_div_falling_edge = self.update_div_falling_edge(div);

    if !self.enabled { return }

    for _ in 0..4 {
      self.channel1.tick();
      self.channel2.tick();
    }

    if is_div_falling_edge {
      self.tick_sequencer();
    }
    
    self.sample_cycles += 4;
    if self.sample_cycles >= AUDIO_CYCLES_PER_SAMPLE {
      self.sample_cycles -= AUDIO_CYCLES_PER_SAMPLE;
      let amplitudes = (
        self.channel1.amplitude(),
        self.channel2.amplitude(),
        0., 0.,
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
      R_NR51 => {
        #[allow(clippy::identity_op, clippy::erasing_op)] {
          seq!(N in 0..4 {
            0 
            #(| ((self.terminals.0.enabled_channels.N as u8) << N))*
            #(| ((self.terminals.1.enabled_channels.N as u8) << (N + 4)))*
          })
        }
      }
      R_NR52 => (self.enabled as u8) << 7, //TODO other NR52 bits
      _ => 0
    }
  }

  pub fn write(&mut self, addr: u16, value: u8, blocking: bool) {
    //If the APU is disabled most registers are R/O
    if blocking && !self.check_write_access(addr) { return }
    match addr {
      R_NR10 | R_NR11 | R_NR12 | R_NR13 | R_NR14 => {
        self.channel1.write(addr, value);
      },
      R_NR21 | R_NR22 | R_NR23 | R_NR24 => {
        self.channel2.write(addr, value);
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
