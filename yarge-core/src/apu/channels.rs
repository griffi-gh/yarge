pub trait ApuChannel {
  fn tick(&mut self) {}
  fn tick_length(&mut self) {}
  fn tick_envelope(&mut self) {}
  fn tick_sweep(&mut self) {}
  fn amplitude(&self) -> f32 { 0. }
  #[allow(unused)] fn read_register(&self, reg: u8) -> u8 { 0 }
  #[allow(unused)] fn write_register(&mut self, reg: u8, value: u8) {}
}

pub mod square;
