use crate::consts::{AUDIO_CYCLES_PER_SAMPLE, audio_registers::*};

mod channels;
mod audio_buffer;
mod audio_device;
mod frame_sequencer;
mod terminal;
mod traits;
mod wave;

pub use traits::ApuChannel;
pub use audio_device::AudioDevice;
use audio_buffer::AudioBuffer;
use channels::square::{SquareWaveChannel, SquareWaveChannelType};
use frame_sequencer::FrameSequencer;
use terminal::Terminal;

pub struct Apu {
  pub device: Option<Box<dyn AudioDevice>>,
  enabled: bool,
  buffer: AudioBuffer,
  sequencer: FrameSequencer,
  channel1: SquareWaveChannel,
  channel2: SquareWaveChannel,
  sample_cycles: usize,
  /// 0 - Right/SO1
  /// 1 - Left /SO2
  terminals: (Terminal, Terminal)
}
impl Apu {
  pub fn new() -> Self {
    Self {
      device: None,
      enabled: false,
      buffer: AudioBuffer::new(),
      sequencer: FrameSequencer::new(),
      channel1: SquareWaveChannel::new(SquareWaveChannelType::Channel1),
      channel2: SquareWaveChannel::new(SquareWaveChannelType::Channel2),
      sample_cycles: 0,
      terminals: (Terminal::new(), Terminal::new())
    }
  }
  pub fn tick(&mut self) {
    if !self.enabled { return }
    self.channel1.tick();
    self.channel2.tick();
    match self.sequencer.tick() {
      Some(0 | 4) => { // Length only
        self.channel1.tick_length();
        self.channel2.tick_length();
      },
      Some(2 | 6) => { // Length and sweep 
        self.channel1.tick_length();
        self.channel2.tick_length();
        self.channel1.tick_sweep();
      },
      Some(7) => { //Envelope only
        self.channel1.tick_envelope();
      },
      _ => ()
    }
    self.sample_cycles += 1;
    if self.sample_cycles >= AUDIO_CYCLES_PER_SAMPLE {
      self.sample_cycles = 0;
      let amplitudes = (
        self.channel1.amplitude(),
        self.channel2.amplitude(),
        0., 0.
      );
      let samples = (
        self.terminals.0.mix_outputs(amplitudes),
        self.terminals.1.mix_outputs(amplitudes),
      );
      self.buffer.push(samples.1, samples.0);
      if self.buffer.is_full() {
        if let Some(device) = self.device.as_mut() {
          device.queue_samples(self.buffer.get_buffer());
          self.buffer.reset();
        }
      }
    }
  }
  fn check_write_access(&self, addr: u16) -> bool {
    self.enabled ||
    [R_NR52, R_NR11, R_NR21, R_NR31, R_NR41].contains(&addr) || //GBC: THIS IS NOT THE CASE ON GBC
    (0xff30..=0xff3f).contains(&addr) // Wave pattern ram
  }
  pub fn write(&mut self, addr: u16, value: u8, blocking: bool) {
    //If the APU is disabled most registers are R/O
    if blocking && !self.check_write_access(addr) { return }
    match addr {
      R_NR52 => {
        self.enabled = (value & 0x80) != 0;
      },
      _ => ()
    }
  }
  pub fn read(&self, addr: u16) -> u8 {
    match addr {
      R_NR52 => (self.enabled as u8) << 7, //TODO other NR52 bits
      _ => 0
    }
  }
}
impl Default for Apu {
  fn default() -> Self { Self::new() }
}
