use arraydeque::ArrayDeque;
use super::{ppu_registers::Lcdc, oam::OamBuffer};
use crate::consts::VRAM_SIZE;

#[derive(Default)]
pub struct FifoPixel {
  pub color: u8,
  //priority: bool,
  //pal: u8,
}
impl FifoPixel {
  pub fn from_color(color: u8) -> Self {
    #[cfg(debug_assertions)]
    assert!(color < 4, "Invalid color");
    Self {
      color, 
      ..Default::default()
    }
  }
}

#[derive(PartialEq)]
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

#[derive(PartialEq)]
#[repr(u8)]
pub enum FetcherLayer {
  Background, Window
}

pub mod fetcher_type {
  pub const SPRITE: bool = true;
  pub const BACKGROUND: bool = false;
}

pub struct Fetcher<const IS_SPRITE: bool> {
  state: FetcherState,
  cycle: bool,
  fifo: Box<ArrayDeque<[FifoPixel; 8]>>,
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
impl<const IS_SPRITE: bool> Fetcher<IS_SPRITE> {
  pub fn new() -> Self { 
    Self {
      cycle: false,
      state: FetcherState::default(),
      fifo: Box::new(ArrayDeque::default()),
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
  pub fn switch_to_window(&mut self) -> bool {
    if self.is_window() {
      return false;
    }
    self.layer = FetcherLayer::Window;
    self.cycle = false;
    self.tile_idx = 0;
    self.offset = 0;
    self.state = FetcherState::ReadTileId;
    self.fifo.clear();
    true
  }
  pub fn is_window(&self) -> bool {
    self.layer == FetcherLayer::Window
  }
  pub fn update(&mut self, scx: u8, scy: u8) {
    self.scx = scx;
    self.scy = scy;
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
        if IS_SPRITE {
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
        } else {
          //self.tile_idx = ;
          todo!();
        }
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
        self.state = FetcherState::PushToFifo;
      },
      FetcherState::PushToFifo => {
        if self.fifo.len() == 0 {
          for x in (0..8_u8).rev() {
            let mask: u8 = 1 << x;
            let (l_bit, h_bit) = (
              ((self.tile_data.0 & mask) != 0) as u8,
              ((self.tile_data.1 & mask) != 0) as u8
            );
            let color = ((h_bit) << 1) | l_bit;
            self.push(
              FifoPixel::from_color(color)
            ).unwrap();
          }
          self.offset += 1;
          self.cycle = false;
          self.state = FetcherState::ReadTileId;
        }
      },
      _ => { self.cycle = true; }
    }
  }

  fn push(&mut self, elem: FifoPixel) -> Result<(), arraydeque::CapacityError<FifoPixel>> {
    self.fifo.push_back(elem)
  }
  pub fn pop(&mut self) -> Option<FifoPixel> {
    self.fifo.pop_front()
  }
  pub fn len(&self) -> usize {
    self.fifo.len()
  }
}
