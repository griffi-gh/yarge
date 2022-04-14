use arraydeque::ArrayDeque;
use super::ppu_registers::LCDC;
use crate::{consts::{TILE_WIDTH, VRAM_SIZE, VRAM_MAX}};

#[derive(Default, Debug)]
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

#[derive(Debug)]
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

#[derive(PartialEq, Debug)]
#[repr(u8)]
pub enum FetcherLayer {
  Background, Window
}

#[derive(Debug)]
pub struct Fetcher {
  cycle: bool,
  state: FetcherState,
  fifo: ArrayDeque<[FifoPixel; 16]>,
  scx: u8, 
  scy: u8,
  ly: u8,
  offset: u16,
  tile_idx: u16,
  tile_data: (u8, u8),
  layer: FetcherLayer,
}
impl Fetcher {
  pub fn new() -> Self { 
    Self {
      cycle: false,
      state: FetcherState::default(),
      fifo: ArrayDeque::default(),
      scx: 0, 
      scy: 0,
      ly: 0,
      offset: 0,
      tile_idx: 0,
      tile_data: (0, 0),
      layer: FetcherLayer::Background,
    }
  }
  pub fn start(&mut self, scx: u8, scy: u8, ly: u8, layer: FetcherLayer) {
    self.scx = scx;
    self.scy = scy;
    self.ly = ly;
    self.layer = layer;
    //
    self.tile_idx = 0;
    self.offset = 0;
    self.cycle = false;
    self.fifo.clear();
    self.state = FetcherState::ReadTileId;
  }
  pub fn tick(&mut self, lcdc: &LCDC, vram: &[u8; VRAM_SIZE]) {
    //run only on every second cycle 
    self.cycle ^= true; //toggle self.cycle
    if self.cycle { return; } //if self.cycle *was* false, skip this cycle
    let get_addr = || {
      self.tile_idx as usize
      //((self.tile_idx << 4) + ((self.y & 7) as u16) << 1) as usize
    };
    match self.state {
      FetcherState::ReadTileId => {
        let map_address = match self.layer {
          FetcherLayer::Background => lcdc.bg_tilemap_addr(),
          FetcherLayer::Window => lcdc.win_tilemap_addr(),
        };

        /*let row = (self.y / 8) as u16;
        let col = (((self.x as u32 + self.offset as u32 * 8) & 0xff) / 8) as u16;

        let addr_in_tilemap = map_address + ((row << 5) + col);*/

        let mut addr: u16 = self.offset;
        if self.layer == FetcherLayer::Background {
          let scy_ly_sum = (self.scy as u16 + self.ly as u16) & 0xff;
          addr += (self.scx as u16 / 8) & 0x1f;
          addr += 32 * (scy_ly_sum / 8);
        }
        addr &= 0x3ff;
        let addr = addr + map_address - 0x8000;
        let tile_idx_raw = vram[addr as usize];
        self.tile_idx = lcdc.transform_tile_index(tile_idx_raw);
        self.tile_data = (0, 0);
        self.state = FetcherState::ReadTileDataLow;
      },
      FetcherState::ReadTileDataLow => {
        /*println!("{:?}", &*self);
        println!("{:#06X}", get_addr());*/
        self.tile_data.0 = vram[get_addr()];
        self.state = FetcherState::ReadTileDataHigh;
      },
      FetcherState::ReadTileDataHigh => {
        self.tile_data.1 = vram[get_addr() + 1];
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        if self.fifo.len() <= 8 {
          for x in (0..TILE_WIDTH).rev() {
            let mask: u8 = 1 << x;
            let (l_bit, h_bit) = (
              (self.tile_data.0 & mask != 0) as u8,
              (self.tile_data.1 & mask != 0) as u8
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
