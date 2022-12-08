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

const MINI_DISPLAY_SIZE: (u32, u32) = (86, 86);
const MINI_DISPLAY_POS:  (i32, i32) = (10, 10);
const TOP_DETAILS_PADDING: (u32, u32) = (10, 0);
const MENU_MARGIN: i32 = 2;

fn menu_item(
  text: &str,
  position: (i32, i32, u32, u32),
  canvas: &mut Canvas<Window>,
  text_renderer: &mut TextRenderer,
  cursor: &mut usize,
  index: usize,
  click: bool,
) -> bool {
  const H_PADDING: u32 = 2;
  let v_padding = (position.3 as i32 - text_renderer.char_size(1.).1 as i32).max(0) as u32 / 2;
  canvas.set_clip_rect(Some(Rect::from(position)));
  if index == *cursor {
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 96));
    canvas.fill_rect(Rect::from(position)).unwrap();
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 128));
    canvas.draw_rect(Rect::from(position)).unwrap();
    text_renderer.set_color(Color::RGBA(255, 255, 255, 255));
  } else {
    text_renderer.set_color(Color::RGBA(0, 0, 0, 255));
  }
  text_renderer.render(canvas, (
    position.0 + H_PADDING as i32,
    position.1 + v_padding as i32
  ), 1., text);
  canvas.set_clip_rect(None);
  false
}

enum MenuLocation {
  MainMenu
}

pub struct Menu {
  active: bool,
  activation_anim_state: Animatable,
  cursor: usize,
  menu_stack: Vec<MenuLocation>
}
impl Menu {
  pub fn new() -> Self {
    Self {
      active: false,
      activation_anim_state: Animatable::new(),
      cursor: 0,
      menu_stack: vec![MenuLocation::MainMenu]
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
      const ANIM_DIST: f32 = 25.;
      let opa = (self.activation_anim_state.value * 255.) as u32 as u8;
      let x_anim_offset = ANIM_DIST - (self.activation_anim_state.value * ANIM_DIST);
      let x_pos = MINI_DISPLAY_POS.0 as i32 + MINI_DISPLAY_SIZE.0 as i32 + TOP_DETAILS_PADDING.0 as i32;
      let y_pos = MINI_DISPLAY_POS.1 as i32 + TOP_DETAILS_PADDING.1 as i32;
      //Game title
      text.set_color(Color::RGBA(0, 0, 0, opa));
      text.render(
        canvas, 
        (x_pos + x_anim_offset as i32, y_pos), 
        2.0,
        gb.get_rom_header().name.as_str()
      );
      //"Paused" text
      text.set_color(Color::RGBA(64, 64, 64, opa));
      text.render(
        canvas, 
        (
          x_pos + (2. * x_anim_offset) as i32,
          y_pos + text.char_size(2.).1 as i32
        ), 
        1.0,
        "Paused"
      );
    }
    //Menu items
    {
      //Macros to display menu items conviniently
      let mut x_position: (i32, i32, u32, u32) = (0, 100, res.0, 18);
      let mut x_index = 0;
      macro_rules! define_menu_item {
        ($text: expr, $on_click: block) => {{
          if menu_item($text, x_position, canvas, text, &mut self.cursor, x_index, false) {
            $on_click;
          }
          x_position.1 += x_position.3 as i32 + MENU_MARGIN;
          x_index += 1;
          let _ = x_index;
        };};
      }
      macro_rules! define_submenu_item {
        ($text: expr, $target: expr) => {{
          define_menu_item!($text, {
            self.menu_stack.push($target);
            self.cursor = 0;
          });
        };};
      }
      //If menu stack contains more then 1 item allow going back
      if self.menu_stack.len() > 1 {
        define_menu_item!("Back", {
          self.menu_stack.pop();
          self.cursor = 0;
        });
      }
      //Menu layouts
      match self.menu_stack.last().unwrap() {
        MenuLocation::MainMenu => {
          define_menu_item!("Load ROM file...", {});
          define_menu_item!("Manage savestates...", {});
          define_menu_item!("Options...", {});
          define_menu_item!("Exit", {});
        } 
      }
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
