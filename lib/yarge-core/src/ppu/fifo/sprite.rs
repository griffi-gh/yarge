use arraydeque::ArrayDeque;
use super::{Fifo, FifoPixel, FetcherState};

pub struct SpriteFetcher {
  fifo: Box<ArrayDeque<[FifoPixel; 8]>>,
  state: FetcherState,
  cycle: bool,
}
impl SpriteFetcher {
  pub fn new() -> Self {
    Self {
      fifo: Box::new(ArrayDeque::new()),
      state: FetcherState::default(),
      cycle: false,
    }
  }
}
impl Fifo for SpriteFetcher {
  fn pop(&mut self) -> Option<FifoPixel> {
    self.fifo.pop_front()
  }
  fn len(&self) -> usize {
    self.fifo.len()
  }
}
