use enumflags2::{bitflags, BitFlags, make_bitflags};
use crate::cpu::{Cpu, Interrupt};

#[bitflags]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum JoypSelect {
  Direction = 0b01,
  Action    = 0b10,
}

#[bitflags]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
  Right  = 1 << 0,
  Left   = 1 << 1,
  Up     = 1 << 2,
  Down   = 1 << 3,
  A      = 1 << 4,
  B      = 1 << 5,
  Select = 1 << 6,
  Start  = 1 << 7,
}
impl Default for Key {
  fn default() -> Self { Self::Right } //0
}

fn filter(state: BitFlags<Key>) -> BitFlags<Key> {
  const UP_DOWN: BitFlags<Key>    = make_bitflags!(Key::{Up | Down });
  const LEFT_RIGHT: BitFlags<Key> = make_bitflags!(Key::{Left | Right});
  let mut state = state;
  if state.contains(UP_DOWN) {
    state ^= UP_DOWN;
  }
  if state.contains(LEFT_RIGHT) {
    state ^= LEFT_RIGHT;
  }
  state
}

pub struct Input {
  select: BitFlags<JoypSelect>,
  key_state: BitFlags<Key>,
  interrupt_flag: bool,
}
impl Input {
  pub fn new() -> Self {
    Self {
      select: BitFlags::default(),
      key_state: BitFlags::default(),
      interrupt_flag: false,
    }
  }

  pub fn set_key_state_all(&mut self, state: u8) {
    //TODO check for interrupts
    self.key_state = BitFlags::from_bits_truncate(state);
  }
  pub fn set_key_state(&mut self, key: Key, state: bool) {
    if state && !self.key_state.contains(key) {
      let input_group = if key as u8 >= (1 << 4) {
        JoypSelect::Action
      } else {
        JoypSelect::Direction
      };
      if self.select.contains(input_group) {
        self.interrupt_flag = true;
      }
    }
    //TODO get rid of this
    self.key_state.remove(key);
    if state {
      self.key_state.insert(key);
    }
  }

  pub fn get_joyp(&self) -> u8 {
    if self.select.bits() == 0 { return 0xFF; }
    let mut output = 0;
    if self.select.contains(JoypSelect::Direction) {
      output |= filter(self.key_state).bits();
    }
    if self.select.contains(JoypSelect::Action) {
      output |= self.key_state.bits() >> 4;
    }
    ((!self.select.bits() & 0b11) << 4) | ((!output) & 0xF) | 0xC0
  }
  pub fn set_joyp(&mut self, value: u8) {
    self.select = BitFlags::from_bits_truncate(!(value >> 4));
  }

  pub fn tick(&mut self, iif: &mut u8) {
    if self.interrupt_flag {
      self.interrupt_flag = false;
      Cpu::set_interrupt(iif, Interrupt::Joypad);
    }
  }
}
