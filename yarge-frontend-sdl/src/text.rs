use sdl2::{
  render::{Texture, Canvas}, 
  video::Window, rect::Rect
};

pub struct TextRenderer<'a> {
  texture: &'a Texture<'a>,
  char_size: (u32, u32),
  chars_per_line: u32,
}
impl<'a> TextRenderer<'a> {
  pub fn new(texture: &'a Texture<'a>, char_size: (u32, u32), chars_per_line: u32) -> Self {
    Self { texture, char_size, chars_per_line }
  }
  fn find_position(&self, char: u8) -> (i32, i32, u32, u32) {
    (
      (((char as u32) % self.chars_per_line) * self.char_size.0) as i32,
      (((char as u32) / self.chars_per_line) * self.char_size.1) as i32,
      self.char_size.0,
      self.char_size.1
    )
  }

  pub fn render(&self, canvas: &mut Canvas<Window>, position: (u32, u32), text: &str) {
    //TODO line breaks
    for (i, char) in text.as_bytes().iter().enumerate() {
      canvas.copy(
        self.texture, 
        Rect::from(self.find_position(*char)), 
        Rect::from((
          (position.0 + ((i as u32) * self.char_size.0)) as i32, 
          position.1 as i32, 
          self.char_size.0, 
          self.char_size.1
        ))
      ).unwrap();
    }
  }
}
