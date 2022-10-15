pub trait ApuChannel {
  fn tick(&mut self); 
  fn tick_length(&mut self) {}
  fn tick_envelope(&mut self) {}
  fn tick_sweep(&mut self) {}
}
