pub trait ApuChannel {
  fn tick(&mut self) {}
  fn tick_length(&mut self) {}
  fn tick_envelope(&mut self) {}
  fn tick_sweep(&mut self) {}
  fn amplitude(&self) -> f32;
  fn read_register(&self, _reg: u8) -> u8 { 0 }
  fn write_register(&mut self, _reg: u8, _value: u8) {}
  fn is_enabled(&self) -> bool;
}

pub mod square;
pub mod noise;
