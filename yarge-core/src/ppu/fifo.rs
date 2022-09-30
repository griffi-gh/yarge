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

pub trait Fifo {
  fn pop(&mut self) -> Option<FifoPixel>;
  fn len(&self) -> usize;
}
