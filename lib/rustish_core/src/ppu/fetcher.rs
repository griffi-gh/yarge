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


#[derive(Clone, Copy, PartialEq)]
enum FetcherState {
  ReadTileId,
  ReadTileDataLow,
  ReadTileDataHigh,
  PushToFifo,
}
impl Default for FetcherState {
  fn default() -> Self { Self::ReadTileId }
}

#[derive(Clone, Copy, PartialEq)]
pub enum FetcherLayer {
  Background, Window
}

pub struct Fetcher {
  cycle: bool,
  state: FetcherState,
  fifo: ArrayDeque<[FifoPixel; 16]>
}
impl Fetcher {
  pub fn new(layer: FetcherLayer) -> Self { 
    Self {
      cycle: false,
      state: FetcherState::default(),
      fifo: ArrayDeque::default(),
    }
  }
  pub fn tick(&mut self) {
    self.cycle ^= true; //toggle self.cycle
    if self.cycle { return; }
    match self.state {
      FetcherState::ReadTileId => {

      },
      FetcherState::ReadTileDataLow => {

      },
      FetcherState::ReadTileDataHigh => {

      },
      FetcherState::PushToFifo => {

      }
    }
  }
}
