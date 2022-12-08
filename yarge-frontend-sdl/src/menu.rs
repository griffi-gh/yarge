use sdl2::{
  event::Event,
  keyboard::Keycode, 
  render::{Canvas, Texture}, 
  video::Window, 
  rect::Rect, 
  pixels::Color, 
};
use crate::{
  anim::Animatable,
  text::TextRenderer
};
use yarge_core::Gameboy;

const MINI_DISPLAY_SIZE: (u32, u32) = (96, 96);
const MINI_DISPLAY_POS:  (i32, i32) = (10, 10);
const TOP_DETAILS_PADDING: (u32, u32) = (10, 0);

pub struct Menu {
  active: bool,
  activation_anim_state: Animatable
}
impl Menu {
  pub fn new() -> Self {
    Self {
      active: false,
      activation_anim_state: Animatable::new(),
    }
  }
  pub fn is_active(&self) -> bool {
    self.active
  }
  pub fn is_visible(&self) -> bool {
    self.is_active() || self.activation_anim_state.is_animating()
  }
  pub fn set_activated_state(&mut self, active: bool) {
    self.activation_anim_state.target = (active as u32) as f32;
    self.active = active;
  }
  ///Process events
  pub fn process_evt(&mut self, event: &Event) {
    match event {
      Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        self.set_activated_state(!self.is_active());
      },
      _ => ()
    }
  }
  pub fn update(
    &mut self,
    canvas: &mut Canvas<Window>,
    gb: &mut Gameboy,
    gb_texture: &Texture,
    text: &mut TextRenderer
  ) {
    //Get canvas resoultion
    let res = canvas.output_size().unwrap(); //HACK: should use logical size, but it returns 0,0
    //Update avtivation animation
    self.activation_anim_state.step();
    //Clear canvas
    canvas.set_draw_color(Color::RGB(233, 226, 207));
    canvas.clear();
    //top details
    {
      text.set_color(Color::RGBA(0, 0, 0, 255));
      text.render(
        canvas, 
        (
          MINI_DISPLAY_POS.0 as u32 + MINI_DISPLAY_SIZE.0 + TOP_DETAILS_PADDING.0,
          MINI_DISPLAY_POS.1 as u32 + TOP_DETAILS_PADDING.1
        ), 
        2.0,
        gb.get_rom_header().name.as_str()
      );
      text.set_color(Color::RGBA(64, 64, 64, 255));
      text.render(
        canvas, 
        (
          MINI_DISPLAY_POS.0 as u32 + MINI_DISPLAY_SIZE.0 + TOP_DETAILS_PADDING.0,
          MINI_DISPLAY_POS.1 as u32 + TOP_DETAILS_PADDING.1 + text.char_size(2.).1
        ), 
        1.0,
        "Paused"
      );
    }
    //Draw display
    {
      let anim = self.activation_anim_state.value;
      let display_pos = (
        (anim * (MINI_DISPLAY_POS.0 as f32)) as i32,
        (anim * (MINI_DISPLAY_POS.1 as f32)) as i32,
        res.0 - (anim * (res.0 - MINI_DISPLAY_SIZE.0) as f32) as u32,
        res.1 - (anim * (res.1 - MINI_DISPLAY_SIZE.1) as f32) as u32,
      );
      //Draw display shadow
      canvas.set_draw_color(Color::RGBA(0, 0, 0, 32));
      canvas.fill_rects(&[
        Rect::from((display_pos.0 - 1, display_pos.1 - 1, display_pos.2 + 2, display_pos.3 + 2)),
        Rect::from((display_pos.0 - 2, display_pos.1 - 2, display_pos.2 + 4, display_pos.3 + 4)),
      ]).unwrap();
      canvas.copy(gb_texture, None, Rect::from(display_pos)).unwrap();
    }
  }
}
impl Default for Menu {
  fn default() -> Self { Self::new() }
}
