use arraydeque::ArrayDeque;
use crate::consts::VRAM_SIZE;
use super::{Fifo, FifoPixel, FetcherState};
use crate::ppu::oam::{OamBuffer, OamObject};

pub struct SpriteFetcher {
  fifo: ArrayDeque<[FifoPixel; 8]>,
  state: FetcherState,
  cycle: bool,
  oam_id: usize,
  object: OamObject, //consider using Option here
  tile_idx: u16,
  tile_data: (u8, u8)
}
impl SpriteFetcher {
  pub fn new() -> Self {
    Self {
      fifo: ArrayDeque::new(),
      state: FetcherState::default(),
      cycle: false,
      oam_id: 0,
      object: OamObject::default(),
      tile_idx: 0,
      tile_data: (0, 0)
    }
  }
  pub fn start(&mut self, oam_id: usize) {
    //self.buffer = buffer.clone();
    debug_assert!(oam_id < 40);
    self.oam_id = oam_id;
    self.cycle = false;
    self.state = FetcherState::ReadTileId;
  }
  pub fn tick(&mut self, buffer: &OamBuffer, vram: &[u8; VRAM_SIZE]) {
    let fetch_addr = self.tile_idx as usize * 16;
    match self.state {
      FetcherState::ReadTileId if self.cycle => {
        self.cycle = false;
        self.object = *buffer.get(self.oam_id).unwrap();
        self.cycle = false;
        self.state = FetcherState::ReadTileDataLow;
      },
      FetcherState::ReadTileDataLow if self.cycle => {
        self.tile_data.0 = vram[fetch_addr];
        self.cycle = false;
        self.state = FetcherState::ReadTileDataHigh;
      },
      FetcherState::ReadTileDataHigh if self.cycle => {
        self.tile_data.1 = vram[fetch_addr + 1];
        self.cycle = false;
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        self.state = FetcherState::ReadTileId;
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
