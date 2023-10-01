pub trait ApuChannel {
  fn tick(&mut self) {}
  fn tick_length(&mut self) {}
  fn tick_envelope(&mut self) {}
  fn tick_sweep(&mut self) {}
  fn amplitude(&self) -> f32 { 0. }
  #[allow(unused)] fn read(&self, mmio_addr: u16) -> u8 { 0 }
  #[allow(unused)] fn write(&mut self, mmio_addr: u16, value: u8) {}
}
