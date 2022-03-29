use arraydeque::ArrayDeque;
use super::VRAM_SIZE;

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


#[repr(u8)]
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
  pub fn tick(&mut self, vram: &[u8; VRAM_SIZE]) {
    //run only on every second cycle 
    self.cycle ^= true; //toggle self.cycle
    if self.cycle { return; } //if self.cycle *was* false, skip this cycle
    match self.state {
      FetcherState::ReadTileId => {
        self.state = FetcherState::ReadTileDataLow;
      },
      FetcherState::ReadTileDataLow => {
        self.state = FetcherState::ReadTileDataHigh;
      },
      FetcherState::ReadTileDataHigh => {
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        if self.fifo.len() <= 8 {
          //TODO
          self.state = FetcherState::ReadTileId;
        }
      }
    }
  }
}
