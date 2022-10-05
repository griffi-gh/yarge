mod background;
mod sprite;
pub use background::BackgroundFetcher;
pub use sprite::SpriteFetcher;

#[derive(Default, Clone, Copy)]
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

pub trait Fetcher {
  fn pop(&mut self) -> Option<FifoPixel>;
  fn len(&self) -> usize;
}

//! DO NOT USE NOT READY YET
#[derive(Default, Clone, Copy)]
pub struct FifoBuffer {
  buffer: [FifoPixel; 8],
  length: usize,
  head: usize,
}
impl FifoBuffer {
  pub fn len(&self) -> usize {
    self.length
  }
  pub fn pop_front(&mut self) -> Option<FifoPixel> {
    if self.head >= self.length {
      return None
    }
    self.length -= 1;
    self.head += 1;
    Some(self.buffer[self.head - 1])
  }
  //TODO use an actual error type
  pub fn push_back(&mut self, value: FifoPixel) -> Result<(),()> { 
    if self.length == 8 {
      return Err(())
    }
    if self.head == 7 {
      self.align();
    }
    self.buffer[self.length] = value;
    self.length += 1;
    Ok(())
  }
  pub fn align(&mut self) {
    self.buffer.rotate_left(self.head);
    self.head = 0;
  }
}
