const EPSILON: f32 = 0.005;

pub struct Animatable {
  pub value: f32,
  pub target: f32,
  pub speed: f32,
}
impl Default for Animatable {
  fn default() -> Self {
    Self {
      value: 0.,
      target: 0.,
      speed: 0.1,
    }
  }
}
impl Animatable {
  pub fn new() -> Self {
    Self::default()
  }
  pub fn new_with_speed(speed: f32) -> Self {
    Self {
      speed,
      ..Default::default()
    }
  }
  pub fn step(&mut self) {
    if (self.target - self.value).abs() < EPSILON {
      self.value = self.target;
    } else {
      self.value = self.value + (self.target - self.value) * self.speed;
    }
  }
  pub fn is_animating(&self) -> bool {
    self.value != self.target
  }
}
