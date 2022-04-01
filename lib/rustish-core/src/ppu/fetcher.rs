use arraydeque::ArrayDeque;
use super::ppu_registers::LCDC;
use crate::consts::{TILE_WIDTH, VRAM_SIZE, VRAM_MAX};

#[derive(Default)]
struct FifoPixel {
  color: u8,
  //priority: bool,
  //pal: u8,
}
impl FifoPixel {
  pub fn from_color(color: u8) -> Self {
    Self {
      color, 
      ..Default::default()
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
  tile: u16,
  tile_data: u16,
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
      tile: 0,
      tile_data: 0,
      layer,
    }
  }
  pub fn tick(&mut self, lcdc: &LCDC, vram: &[u8; VRAM_SIZE]) {
    //run only on every second cycle 
    self.cycle ^= true; //toggle self.cycle
    if self.cycle { return; } //if self.cycle *was* false, skip this cycle
    let get_addr = |is_high: u16| {
      (self.tile << 4) + ((((self.y >> 3) as u16) << 1) | is_high)
    };
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
        self.tile = lcdc.transform_tile_index(tile);
        self.tile_data = 0;
        self.state = FetcherState::ReadTileDataLow;
      },
      FetcherState::ReadTileDataLow => {
        self.tile_data |= vram[(get_addr(0) & VRAM_MAX) as usize] as u16;
        self.state = FetcherState::ReadTileDataHigh;
      },
      FetcherState::ReadTileDataHigh => {
        let addr = get_addr(1);
        self.tile_data |= (vram[(get_addr(1) & VRAM_MAX) as usize] as u16) << 8;
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        if self.fifo.len() <= 8 {
          //TODO
          for x in (0..TILE_WIDTH).rev() {
            let high_bit = (self.tile_data & (1 << x) != 0) as u8;
            let low_bit = ((self.tile_data >> 8) & (1 << x) != 0) as u8;
            let color = (high_bit) << 1 | low_bit;
            self.fifo.push_back(
              FifoPixel::from_color(color)
            ).unwrap();
          }
          self.state = FetcherState::ReadTileId;
        }
      }
    }
  }
}
