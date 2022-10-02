use arraydeque::ArrayDeque;
use bit_reverse::LookupReverse;
use crate::consts::VRAM_SIZE;
use super::{Fifo, FifoPixel, FetcherState};
use crate::ppu::{
  oam::OamObject,
  ppu_registers::Lcdc,
  util, 
};

pub struct SpriteFetcher {
  fifo: ArrayDeque<[FifoPixel; 8]>,
  state: FetcherState,
  cycle: bool,
  object: OamObject,
  tile_idx: usize,
  tile_data: (u8, u8),
  ly: u8,
  pub fetching: bool
}
impl SpriteFetcher {
  pub fn new() -> Self {
    Self {
      fifo: ArrayDeque::new(),
      state: FetcherState::default(),
      cycle: false,
      object: OamObject::default(),
      tile_idx: 0,
      tile_data: (0, 0),
      ly: 0,
      fetching: false
    }
  }
  pub fn eol(&mut self) {
    self.fetching = false;
    self.cycle = false;
    self.state = FetcherState::ReadTileId;
    self.fifo.clear();
  }
  pub fn start(&mut self, object: OamObject, ly: u8) {
    self.object = object;
    self.ly = ly;
    self.cycle = false;
    self.state = FetcherState::ReadTileId;
    self.fetching = true;
  }
  pub fn tick(&mut self, lcdc: &Lcdc, vram: &[u8; VRAM_SIZE]) {
    let fetch_addr = || {
      let mut y_offset = (self.ly as usize + 16) - self.object.y as usize;
      let mut tile_idx = self.tile_idx;
      if lcdc.obj_size {
        tile_idx |= ((y_offset > 7) ^ self.object.flags.flip_y) as usize;
        y_offset &= 7;
      } else if self.object.flags.flip_y {
        y_offset = 7 - y_offset;
      }
      (tile_idx * 16) + (y_offset * 2)
    };
    match self.state {
      FetcherState::ReadTileId if self.cycle => {
        self.cycle = false;
        self.tile_idx = self.object.tile as usize;
        if lcdc.obj_size {
          self.tile_idx &= 0xFE;
        }
        self.state = FetcherState::ReadTileDataLow;
      },
      FetcherState::ReadTileDataLow if self.cycle => {
        self.tile_data.0 = vram[fetch_addr()];
        self.cycle = false;
        self.state = FetcherState::ReadTileDataHigh;
      },
      FetcherState::ReadTileDataHigh if self.cycle => {
        self.tile_data.1 = vram[fetch_addr() + 1];
        self.cycle = false;
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        self.state = FetcherState::ReadTileId;
        //TODO this code is... not very good (tm)
        //Make sure that fifo is filled up
        while !self.fifo.is_full() {
          self.fifo.push_back(FifoPixel::from_color(0)).unwrap();
        }
        //Reverse tile data if flip_x flag is set
        if self.object.flags.flip_x {
          self.tile_data.0 = LookupReverse::swap_bits(self.tile_data.0);
          self.tile_data.1 = LookupReverse::swap_bits(self.tile_data.1);
        }
        //If the object is close to the edge, skip some pixels
        let mut take = 8;
        if self.object.x < 8 {
          let shift = 7 - (self.object.x as u32);
          self.tile_data.0 = self.tile_data.0.wrapping_shl(shift);
          self.tile_data.1 = self.tile_data.1.wrapping_shl(shift);
          take -= shift as usize;
        }
        let colors = util::spr_line(self.tile_data);
        for (i, color) in colors.iter().enumerate().take(take) {
          //Only paint on top if the bg is transparent
          if self.fifo[i].color == 0 {
            self.fifo[i] = FifoPixel {
              color: *color,
              priority: self.object.flags.priority,
              pal: self.object.flags.palette
            };
          }
        };
        self.state = FetcherState::ReadTileId;
        self.fetching = false;
      },
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
