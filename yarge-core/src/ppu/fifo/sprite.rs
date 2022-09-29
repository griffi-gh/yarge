use arraydeque::ArrayDeque;
use super::{Fifo, FifoPixel, FetcherState};
//use crate::ppu::oam::OamBuffer;

pub struct SpriteFetcher {
  fifo: ArrayDeque<[FifoPixel; 8]>,
  state: FetcherState,
  cycle: bool,
}
impl SpriteFetcher {
  pub fn new() -> Self {
    Self {
      fifo: ArrayDeque::new(),
      state: FetcherState::default(),
      cycle: false,
    }
  }
  pub fn start(&mut self) {
    //self.buffer = buffer.clone();
    self.cycle = false;
    self.state = FetcherState::ReadTileId;
  }
  pub fn tick(&mut self) {
    match self.state {
      FetcherState::ReadTileId if self.cycle => {
        self.cycle = false;
        self.state = FetcherState::ReadTileDataLow;
      }
      _ => { self.cycle = true }
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
