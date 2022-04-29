#[repr(u8)]
pub enum Key {
  Up, Down, Left, Right,
  Start, Select, A, B
}

#[derive(Default, Clone, Copy)]
struct KeyState {
  up: bool,
  down: bool,
  left: bool,
  right: bool,
  start: bool,
  select: bool,
  a: bool,
  b: bool
}
impl KeyState {
  fn filter(&self) -> Self {
    Self {
      up:    self.up    && !self.down,
      down:  self.down  && !self.up,
      left:  self.left  && !self.right,
      right: self.right && !self.left,
      ..*self
    }
  }
  pub fn get_key(&self, key: Key) -> bool {
    match key {
      Key::Up     => self.up,
      Key::Down   => self.down,
      Key::Left   => self.left,
      Key::Right  => self.right,
      Key::Start  => self.start,
      Key::Select => self.select,
      Key::A      => self.a,
      Key::B      => self.b,
    }
  }
  pub fn set_key(&mut self, key: Key, state: bool) {
    match key {
      Key::Up     => { self.up     = state },
      Key::Down   => { self.down   = state },
      Key::Left   => { self.left   = state },
      Key::Right  => { self.right  = state },
      Key::Start  => { self.start  = state },
      Key::Select => { self.select = state },
      Key::A      => { self.a      = state },
      Key::B      => { self.b      = state },
    }
  }
}

pub struct Input {
  key_state: KeyState
}
impl Input {
  pub fn new() -> Self {
    Self {
      key_state: KeyState::default()
    }
  }
}
