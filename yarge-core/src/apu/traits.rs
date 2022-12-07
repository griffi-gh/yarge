pub trait ApuChannel {
  fn tick(&mut self) {}
  fn tick_length(&mut self) {}
  fn tick_envelope(&mut self) {}
  fn tick_sweep(&mut self) {}
  fn amplitude(&self) -> f32 { 0. }
  fn read(&self, mmio_addr: u8) -> u8 { 0 }
  fn write(&mut self, mmio_addr: u8, value: u8) {}
}
