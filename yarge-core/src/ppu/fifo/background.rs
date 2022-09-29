use arraydeque::ArrayDeque;
use crunchy::unroll;
use crate::consts::VRAM_SIZE;
use crate::ppu::ppu_registers::Lcdc;
use crate::ppu::util;
use super::{Fifo, FetcherState, FifoPixel};

#[derive(PartialEq, Eq)]
#[repr(u8)]
pub enum FetcherLayer {
  Background, Window
}

pub struct BackgroundFetcher {
  fifo: ArrayDeque<[FifoPixel; 8]>,
  state: FetcherState,
  cycle: bool,
  scx: u8, 
  scy: u8,
  wly: u8,
  ly: u8,
  offset: u16,
  tile_idx: u16,
  tile_data: (u8, u8),
  layer: FetcherLayer,
  sleep: u8,
}
impl BackgroundFetcher {
  pub fn new() -> Self { 
    Self {
      cycle: false,
      state: FetcherState::default(),
      fifo: ArrayDeque::default(),
      scx: 0, 
      scy: 0,
      wly: 0,
      ly: 0,
      offset: 0,
      tile_idx: 0,
      tile_data: (0, 0),
      layer: FetcherLayer::Background,
      sleep: 6,
    }
  }
  pub fn start(&mut self, scx: u8, scy: u8, ly: u8, wly: u8) {
    self.scx = scx;
    self.scy = scy;
    self.ly = ly;
    self.wly = wly;
    self.layer = FetcherLayer::Background;
    self.tile_idx = 0;
    self.offset = 0;
    self.cycle = false;
    self.state = FetcherState::ReadTileId;
    self.sleep = 6;
    self.fifo.clear();
  }
  pub fn switch_to_window(&mut self) {
    debug_assert!(!self.is_window());
    self.layer = FetcherLayer::Window;
    self.cycle = false;
    self.tile_idx = 0;
    self.offset = 0;
    self.state = FetcherState::ReadTileId;
    self.fifo.clear();
  }
  pub fn is_window(&self) -> bool {
    self.layer == FetcherLayer::Window
  }
  pub fn update_values(&mut self, scx: u8, scy: u8) {
    self.scx = scx;
    self.scy = scy;
  }
  pub fn spr_reset(&mut self) {
    self.cycle = false;
    self.state = FetcherState::default();
  }
  pub fn tick(&mut self, lcdc: &Lcdc, vram: &[u8; VRAM_SIZE]) {
    if self.sleep > 0 {
      self.sleep -= 1;
      return;
    }
    let fetch_addr = || {
      let tile = self.tile_idx as usize * 16;
      match self.layer { 
        FetcherLayer::Background => tile + (2 * (self.ly.wrapping_add(self.scy) & 7)) as usize,
        FetcherLayer::Window     => tile + (2 * ((self.wly as usize) & 7)),
      }
    };
    match self.state {
      FetcherState::ReadTileId if self.cycle => {
        let addr: u16 = {
          let mut addr = self.offset;
          match self.layer {
            FetcherLayer::Background => {
              addr += self.scx as u16 >> 3;
              addr &= 0x1f;
              addr += 32 * (self.ly.wrapping_add(self.scy) as u16 >> 3);
            },
            FetcherLayer::Window => {
              //TODO verify
              addr += (self.wly as u16 >> 3) << 5;
            }
          }
          addr + match self.layer {
            FetcherLayer::Background => lcdc.bg_tilemap_addr() - 0x8000,
            FetcherLayer::Window => lcdc.win_tilemap_addr() - 0x8000,
          }
        };
        self.tile_idx = lcdc.transform_tile_index(vram[addr as usize]);
        self.cycle = false;
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
        if self.fifo.is_empty() {
          for color in util::spr_line(self.tile_data) {
            self.fifo.push_back(
              FifoPixel::from_color(color)
            ).unwrap();
          }
          self.offset += 1;
          self.state = FetcherState::ReadTileId;
        }
      },
      _ => { self.cycle = true }
    }
  }
}
impl Fifo for BackgroundFetcher {
  fn pop(&mut self) -> Option<FifoPixel> {
    self.fifo.pop_front()
  }
  fn len(&self) -> usize {
    self.fifo.len()
  }
}
