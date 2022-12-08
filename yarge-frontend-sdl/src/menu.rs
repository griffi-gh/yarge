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

const PAUSED_DISPLAY_SIZE: (u32, u32) = (96, 96);

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
  pub fn update(&mut self, canvas: &mut Canvas<Window>, gb_texture: &Texture, text: &TextRenderer) {
    //Update avtivation animation
    self.activation_anim_state.step();
    //Clear canvas
    canvas.set_draw_color(Color::RGB(233, 226, 207));
    canvas.clear();
    let res = canvas.output_size().unwrap(); //HACK: should use logical size, but it returns 0,0
    //Draw display
    {
      let anim = self.activation_anim_state.value;
      let display_pos = (
        (anim * 10.) as i32,
        (anim * 10.) as i32,
        res.0 - (anim * (res.0 - PAUSED_DISPLAY_SIZE.0) as f32) as u32,
        res.1 - (anim * (res.1 - PAUSED_DISPLAY_SIZE.1) as f32) as u32,
      );
      //Draw display shadow
      canvas.set_draw_color(Color::RGBA(0, 0, 0, 32));
      canvas.fill_rects(&[
        Rect::from((display_pos.0 - 1, display_pos.1 - 1, display_pos.2 + 2, display_pos.3 + 2)),
        Rect::from((display_pos.0 - 2, display_pos.1 - 2, display_pos.2 + 4, display_pos.3 + 4)),
      ]).unwrap();
      canvas.copy(gb_texture, None, Rect::from(display_pos)).unwrap();
    }
    //testing
    text.render(canvas, (0, 0), "testing text");
  }
}
impl Default for Menu {
  fn default() -> Self { Self::new() }
}
