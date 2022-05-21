mod background;
mod sprite;
pub use background::BackgroundFetcher;
pub use sprite::SpriteFetcher;

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

pub trait Fetcher {
  fn pop(&mut self) -> Option<FifoPixel>;
  fn len(&self) -> usize;
}
