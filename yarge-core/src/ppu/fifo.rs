mod background;
mod sprite;
pub use background::BackgroundFetcher;
pub use sprite::SpriteFetcher;

#[derive(Default)]
pub struct FifoPixel {
  pub color: u8,
  pub priority: bool,
  pub pal: bool,
  //pal: u8, (CGB ONLY)
}
impl FifoPixel {
  pub fn from_color(color: u8) -> Self {
    debug_assert!(color < 4, "Invalid color");
    Self {
      color, 
      priority: false,
      pal: false
      //..Default::default()
    }
  }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FetcherState {
  ReadTileId = 0,
  ReadTileDataLow = 1,
  ReadTileDataHigh = 2,
  PushToFifo = 3,
}
impl Default for FetcherState {
  fn default() -> Self { Self::ReadTileId }
}

pub trait Fifo {
  fn pop(&mut self) -> Option<FifoPixel>;
  fn len(&self) -> usize;
}
