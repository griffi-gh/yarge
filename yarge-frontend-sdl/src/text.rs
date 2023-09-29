use sdl2::{
  render::{Texture, Canvas}, 
  video::Window,
  rect::Rect, 
  pixels::Color
};

pub struct TextRenderer<'a> {
  texture: Texture<'a>,
  char_size: (u32, u32),
  chars_per_line: u32,
  dpi_scale: f32,
}
impl<'a> TextRenderer<'a> {
  pub fn new(texture: Texture<'a>, char_size: (u32, u32), chars_per_line: u32) -> Self {
    Self { texture, char_size, chars_per_line, dpi_scale: 1. }
  }

  fn find_position(&self, char: u8) -> (i32, i32, u32, u32) {
    (
      (((char as u32) % self.chars_per_line) * self.char_size.0) as i32,
      (((char as u32) / self.chars_per_line) * self.char_size.1) as i32,
      self.char_size.0,
      self.char_size.1
    )
  }

  pub fn set_color(&mut self, color: Color) {
    self.texture.set_color_mod(color.r, color.g, color.b);
    self.texture.set_alpha_mod(color.a);
  }

  pub fn set_render_dpi_scale(&mut self, scale: f32) {
    self.dpi_scale = scale;
  }

  pub fn render(&self, canvas: &mut Canvas<Window>, position: (i32, i32), size: f32, text: &str) {
    let mut col = 0;
    let mut line = 0;
    for &char in text.as_bytes() {
      if char == b'\n' {
        line += 1;
        col = 0;
        continue;
      }
      canvas.copy(
        &self.texture, 
        Rect::from(self.find_position(char)), 
        Rect::from((
          (position.0 as f32 + (col as f32 * self.char_size.0 as f32 * size * self.dpi_scale)) as i32,
          (position.1 as f32 + (line as f32 * self.char_size.1 as f32 * size * self.dpi_scale)) as i32,
          (self.char_size.0 as f32 * size * self.dpi_scale) as u32, 
          (self.char_size.1 as f32 * size * self.dpi_scale) as u32
        ))
      ).unwrap();
      col += 1;
    }
  }

  pub fn char_size(&self, size: f32) -> (u32, u32) {
    (
      (self.char_size.0 as f32 * size) as u32,
      (self.char_size.1 as f32 * size) as u32
    )
  }

  pub fn text_size(&self, text: &str, size: f32) -> (u32, u32) {
    //TODO handle newlines
    (
      ((text.len() as u32 * self.char_size.0) as f32 * size) as u32,
      (self.char_size.1 as f32 * size) as u32
    )
  }
}
