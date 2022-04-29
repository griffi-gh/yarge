#[repr(u8)]
enum Key {
  Up, Down, Left, Right,
  Start, Select, A, B
}

#[derive(Clone, Copy)]
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
  fn get_key_mut(&mut self, key: Key) -> &mut bool {
    match key {
      Key::Up     => &mut self.up,
      Key::Down   => &mut self.down,
      Key::Left   => &mut self.left,
      Key::Right  => &mut self.right,
      Key::Start  => &mut self.start,
      Key::Select => &mut self.select,
      Key::A      => &mut self.a,
      Key::B      => &mut self.b,
    }
  }
  fn filter(&self) -> Self {
    Self {
      up:    self.up    && !self.down,
      down:  self.down  && !self.up,
      left:  self.left  && !self.right,
      right: self.right && !self.left,
      ..self
    }
  }

  pub fn get_key(&self, key: Key) -> bool {
    *self.get_key_mut(key)
  }
  pub fn get_key_filtered(&self, key: Key) -> bool {
    *self.filter().get_key_mut(key)
  }

  pub fn set_key(&mut self, key: Key, state: bool) {
    *self.get_key_mut(key) = state;
  }
}

struct Input {
  key_state: KeyState
}
