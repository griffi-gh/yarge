use arraydeque::ArrayDeque;
use super::{VRAM_SIZE, VRAM_MAX, ppu_registers::LCDC};

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
  fifo: ArrayDeque<[FifoPixel; 16]>,
  x: u8, y: u8,
  offset: u16,
  layer: FetcherLayer,
}
impl Fetcher {
  pub fn new(layer: FetcherLayer) -> Self { 
    Self {
      cycle: false,
      state: FetcherState::default(),
      fifo: ArrayDeque::default(),
      x: 0, y: 0,
      offset: 0,
      layer,
    }
  }
  pub fn tick(&mut self, lcdc: &LCDC, vram: &[u8; VRAM_SIZE]) {
    //run only on every second cycle 
    self.cycle ^= true; //toggle self.cycle
    if self.cycle { return; } //if self.cycle *was* false, skip this cycle
    match self.state {
      FetcherState::ReadTileId => {
        let map_address = match self.layer {
          FetcherLayer::Background => lcdc.bg_tilemap_addr(),
          FetcherLayer::Window => lcdc.win_tilemap_addr(),
        };
        let row = (self.y >> 3) as u16;
        let col = ((self.x as u16 + self.offset << 3) & 0xFF) << 3;
        let addr_in_tilemap = map_address + (row << 5 + col);
        let tile = vram[(addr_in_tilemap & VRAM_MAX) as usize];
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
