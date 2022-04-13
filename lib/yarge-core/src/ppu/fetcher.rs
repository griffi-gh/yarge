use arraydeque::ArrayDeque;
use super::ppu_registers::LCDC;
use crate::{consts::{TILE_WIDTH, VRAM_SIZE, VRAM_MAX}};

#[derive(Default)]
pub struct FifoPixel {
  pub color: u8,
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
enum FetcherState {
  ReadTileId,
  ReadTileDataLow,
  ReadTileDataHigh,
  PushToFifo,
}
impl Default for FetcherState {
  fn default() -> Self { Self::ReadTileId }
}

#[repr(u8)]
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
  pub fn new() -> Self { 
    Self {
      cycle: false,
      state: FetcherState::default(),
      fifo: ArrayDeque::default(),
      x: 0, y: 0,
      offset: 0,
      tile: 0,
      tile_data: 0,
      layer: FetcherLayer::Background,
    }
  }
  pub fn start(&mut self, x: u8, y: u8, layer: FetcherLayer) {
    self.x = x;
    self.y = y;
    self.layer = layer;
    //
    self.tile = 0;
    self.offset = 0;
    self.cycle = false;
    self.fifo.clear();
    self.state = FetcherState::ReadTileId;
  }
  pub fn tick(&mut self, lcdc: &LCDC, vram: &[u8; VRAM_SIZE]) {
    //run only on every second cycle 
    self.cycle ^= true; //toggle self.cycle
    if self.cycle { return; } //if self.cycle *was* false, skip this cycle
    let get_addr = |is_high: u16| {
      /*let tile_number = self.tile;
      let tile_row = (self.y % 8) as usize;
      // 2x = 2 bytes per line
      let line_base = tile_row * 2;
      let line_offset = line_base + is_high as usize;
      let tile_mask = tile_number << 4;
      tile_mask as usize + line_offset*/
      ((self.tile << 4) + ((((self.y >> 3) as u16) << 1) | is_high)) as usize
    };
    match self.state {
      FetcherState::ReadTileId => {
        let map_address = match self.layer {
          FetcherLayer::Background => lcdc.bg_tilemap_addr(),
          FetcherLayer::Window => lcdc.win_tilemap_addr(),
        };

        let row = (self.y / 8) as u16;
        let col = (((self.x as u32 + self.offset as u32 * 8) & 0xff) / 8) as u16;

        let addr_in_tilemap = map_address + ((row << 5) + col);
        let tile = vram[(addr_in_tilemap & VRAM_MAX) as usize];
        self.tile = lcdc.transform_tile_index(tile);
        self.tile_data = 0;
        self.state = FetcherState::ReadTileDataLow;
      },
      FetcherState::ReadTileDataLow => {
        self.tile_data |= vram[(get_addr(0) & VRAM_MAX as usize)] as u16;
        self.state = FetcherState::ReadTileDataHigh;
      },
      FetcherState::ReadTileDataHigh => {
        self.tile_data |= (vram[(get_addr(1) & VRAM_MAX as usize)] as u16) << 8;
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        if self.fifo.len() <= 8 {
          for x in (0..TILE_WIDTH).rev() {
            let mask: u8 = 1 << x;
            let (h_data, l_data) = (
              (self.tile_data >> 8) as u8,
              self.tile_data as u8
            );
            let (h_bit, l_bit) = (
              (h_data & mask != 0) as u8,
              (l_data & mask != 0) as u8
            );
            let color = (h_bit) << 1 | l_bit;
            self.push(
              FifoPixel::from_color(color)
            ).unwrap();
          }
          self.offset += 1;
          self.state = FetcherState::ReadTileId;
        }
      }
    }
  }

  #[inline] fn push(&mut self, elem: FifoPixel) -> Result<(), arraydeque::CapacityError<FifoPixel>> {
    self.fifo.push_back(elem)
  }

  #[inline] pub fn pop(&mut self) -> Option<FifoPixel> {
    self.fifo.pop_front()
  }
  #[inline] pub fn len(&self) -> usize {
    self.fifo.len()
  }
}
