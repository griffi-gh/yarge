use arraydeque::ArrayDeque;

struct FifoPixel {
  color: u8,
  priority: bool,
  pal: u8,
}
impl Default for FifoPixel {
  fn default() -> Self {
    Self {
      color: 0,
      priority: false,
      pal: 0,
    }
  }
}

pub struct Fetcher {
  fifo: ArrayDeque<[FifoPixel; 16]>
}
impl Fetcher {
  pub fn new() -> Self { 
    Self {
      fifo: ArrayDeque::default()
    }
  }
  pub fn tick(&mut self) {
    
  }
}
