use arraydeque::ArrayDeque;
use super::{Fifo, FifoPixel, FetcherState};
//use crate::ppu::oam::OamBuffer;

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
  pub fn start(&mut self) {
    //self.buffer = buffer.clone();
    self.cycle = false;
    self.state = FetcherState::ReadTileId;
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
