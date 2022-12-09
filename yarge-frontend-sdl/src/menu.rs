use sdl2::{
  event::Event,
  keyboard::Keycode, 
  render::{Canvas, Texture}, 
  video::Window, 
  rect::Rect, 
  pixels::Color, 
};
use std::borrow::Cow;
use yarge_core::Gameboy;
use crate::{
  anim::Animatable,
  text::TextRenderer
};


const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const MINI_DISPLAY_SIZE: (u32, u32) = (86, 86);
const MINI_DISPLAY_POS:  (i32, i32) = (10, 10);
const TOP_DETAILS_PADDING: (u32, u32) = (10, 0);
const MENU_MARGIN: i32 = 2;
const MENU_ITEM_H_PADDING: u32 = 4;
const MENU_ITEM_HEIGHT: u32 = 18;
const SHORT_PATH_CHARS: usize = 2;

fn menu_item(
  text: &str,
  position: (i32, i32, u32, u32),
  canvas: &mut Canvas<Window>,
  text_renderer: &mut TextRenderer,
  cursor: isize,
  index: usize,
  click: bool,
) -> bool {
  let v_padding = (position.3 as i32 - text_renderer.char_size(1.).1 as i32).max(0) as u32 / 2;
  canvas.set_clip_rect(Some(Rect::from(position)));
  if index as isize == cursor {
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 96));
    canvas.fill_rect(Rect::from(position)).unwrap();
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 128));
    canvas.draw_rect(Rect::from(position)).unwrap();
    text_renderer.set_color(Color::RGBA(255, 255, 255, 255));
  } else {
    text_renderer.set_color(Color::RGBA(0, 0, 0, 255));
  }
  text_renderer.render(canvas, (
    position.0 + MENU_ITEM_H_PADDING as i32,
    position.1 + v_padding as i32
  ), 1., text);
  canvas.set_clip_rect(None);
  click && (index as isize == cursor)
}

#[derive(Clone, Copy)]
enum MenuLocation {
  Main,
  Options,
  PalettePicker
}
impl MenuLocation {
  pub fn friendly_name(&self) -> &'static str {
    match *self {
      MenuLocation::Main => "Main menu",
      MenuLocation::Options => "Options",
      MenuLocation::PalettePicker => "Color palette"
    }
  }
}

pub struct Menu {
  active: bool,
  activation_anim_state: Animatable,
  cursor: isize,
  menu_stack: Vec<MenuLocation>,
  clicked: bool
}
impl Menu {
  pub fn new() -> Self {
    Self {
      active: false,
      activation_anim_state: Animatable::new(),
      cursor: 0,
      menu_stack: vec![MenuLocation::Main],
      clicked: false,
    }
  }
  pub fn is_active(&self) -> bool {
    self.active
  }
  pub fn is_visible(&self) -> bool {
    self.is_active() || self.activation_anim_state.is_animating()
  }
  pub fn set_activated_state(&mut self, active: bool) {
    if (!self.is_visible()) && (!self.active) && active {
      //reset menu
      self.cursor = 0;
      self.menu_stack = vec![MenuLocation::Main];
    }
    self.activation_anim_state.target = (active as u32) as f32;
    self.active = active;
  }
  ///Process events
  pub fn process_evt(&mut self, event: &Event) {
    match event {
      Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
        self.set_activated_state(!self.is_active());
      },
      Event::KeyDown { keycode: Some(Keycode::Down), .. } if self.active => {
        self.cursor += 1;
      },
      Event::KeyDown { keycode: Some(Keycode::Up), .. } if self.active => {
        self.cursor -= 1;
      },
      Event::KeyDown { keycode: Some(Keycode::Return | Keycode::Return2), repeat: false, .. } if self.active => {
        self.clicked = true;
      },
      _ => ()
    }
  }
  pub fn update(
    &mut self,
    canvas: &mut Canvas<Window>,
    gb: &mut Gameboy,
    gb_texture: &Texture,
    text: &mut TextRenderer,
    do_exit: &mut bool,
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
      let x_pos = MINI_DISPLAY_POS.0 + MINI_DISPLAY_SIZE.0 as i32 + TOP_DETAILS_PADDING.0 as i32;
      let y_pos = MINI_DISPLAY_POS.1 + TOP_DETAILS_PADDING.1 as i32;
      let display_name = gb.get_rom_header().name;
      let computed_scale = ((res.0 as f32 - x_pos as f32) / (display_name.len() as f32 * text.char_size(1.).0 as f32)).min(2.);
      //Game title
      text.set_color(Color::RGBA(0, 0, 0, opa));
      text.render(
        canvas, 
        (x_pos + x_anim_offset as i32, y_pos), 
        computed_scale,
        display_name.as_str()
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
      //Menu path
      let path: Cow<str> = if self.menu_stack.len() <= 1 {
        Cow::from(self.menu_stack.last().unwrap().friendly_name())
      } else {
        let capacity = (SHORT_PATH_CHARS + 1) * (self.menu_stack.len() - 1) + self.menu_stack.last().unwrap().friendly_name().len();
        let mut path = String::with_capacity(capacity);
        for item in &self.menu_stack[..self.menu_stack.len() - 1] {
          let name = item.friendly_name();
          let words = name.matches(' ').count() + 1;
          if words >= SHORT_PATH_CHARS {
            for word in name.split(' ').take(SHORT_PATH_CHARS) {
              path.push(word.chars().next().unwrap());
            }
          } else {
            path += &name[..SHORT_PATH_CHARS];
          }
          path.push('>');
        }
        path += self.menu_stack.last().unwrap().friendly_name();
        debug_assert!(capacity == path.len());
        Cow::from(path)
      };
      text.render(
        canvas, 
        (
          x_pos + (2. * x_anim_offset) as i32,
          MINI_DISPLAY_POS.1 + MINI_DISPLAY_SIZE.1 as i32 - (text.char_size(1.).1 as i32) - TOP_DETAILS_PADDING.1 as i32
        ), 
        1.,
        path.as_ref()
      );
    }
    //Menu items
    {
      //Macros to display menu items conviniently
      let list_start_y = (MINI_DISPLAY_POS.1 << 1) + MINI_DISPLAY_SIZE.1 as i32;
      let mut x_position: (i32, i32, u32, u32) = (0, list_start_y, res.0, MENU_ITEM_HEIGHT);
      let mut x_index = 0;
      macro_rules! define_menu_item {
        ($text: expr, $on_click: block) => {{
          if menu_item($text, x_position, canvas, text, self.cursor, x_index, self.clicked) {
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
      macro_rules! define_spacing_item {
        ($pixels: expr) => {
          x_position.1 += $pixels as i32;
        };
      }
      //Get top item from the menu stack
      let top_item = *self.menu_stack.last().unwrap();
      //If menu stack contains more then 1 item allow going back
      if self.menu_stack.len() > 1 {
        define_menu_item!("Back", {
          self.menu_stack.pop();
          self.cursor = 0;
        });
        define_spacing_item!(MENU_MARGIN);
      }
      //Menu layouts
      match top_item {
        MenuLocation::Main => {
          define_menu_item!("Resume", {
            self.set_activated_state(false);
          });
          define_menu_item!("Load ROM file...", {});
          // define_menu_item!("Manage savestates...", {});
          define_submenu_item!("Options...", MenuLocation::Options);
          define_menu_item!("Exit", { *do_exit = true });
        },
        MenuLocation::Options => {
          define_submenu_item!("Color palette...", MenuLocation::PalettePicker);
          define_menu_item!("Display scale...", {});
        },
        MenuLocation::PalettePicker => {
          define_menu_item!("> Grayscale", {});
          define_menu_item!("BGB", {});
        }
        _ => ()
      }
      //HACK: Limit cursor
      if self.cursor < 0 {
        self.cursor = x_index as isize - 1;
      } else if self.cursor >= x_index as isize {
        self.cursor = 0;
      }
    }
    //Draw key help
    {
      //height
      let h = text.char_size(1.).1 + 2;
      //animation
      let opa = (self.activation_anim_state.value * 255.) as u32 as u8;
      let offst = h as i32 - (self.activation_anim_state.value * h as f32) as i32;
      //box
      canvas.set_draw_color(Color::RGBA(0, 0, 0, opa / 4));
      canvas.fill_rects(&[
        Rect::from((0, res.1 as i32 - h as i32 - 1 + offst, res.0, h + 1)),
        Rect::from((0, res.1 as i32 - h as i32 + offst, res.0, h))
      ]).unwrap();
      //compute y coord
      let text_y = res.1 as i32 - h as i32 + 1 + offst;
      //help text
      text.set_color(Color::RGBA(255, 255, 255, opa));
      text.render(canvas, (3, text_y), 1., "\x1e\x1f Move cursor \x04 Confirm");
      //version text
      let ver_x = (res.0 - text.text_size(CRATE_VERSION, 1.).0 - 3) as i32;
      text.set_color(Color::RGBA(255, 255, 255, opa / 3));
      text.render(canvas, (ver_x, text_y), 1., CRATE_VERSION);
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
    self.clicked = false;
  }
}
impl Default for Menu {
  fn default() -> Self { Self::new() }
}
