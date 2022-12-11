use sdl2::{
  event::Event,
  keyboard::Keycode, 
  render::{Canvas, Texture}, 
  video::{Window, FullscreenType}, 
  rect::Rect, 
  pixels::Color, 
};
use std::{
  borrow::Cow, 
  path::{PathBuf, Path}, 
  fs::{self, DirEntry}
};
use yarge_core::{
  Gameboy, 
  consts::{
    WIDTH as GB_WIDTH,
    HEIGHT as GB_HEIGHT
  }
};
use crate::{
  anim::Animatable,
  text::TextRenderer,
  config::{Configuration, Palette, WindowScale}
};

const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
const BACKGROUND_COLOR: Color = Color::RGB(233, 226, 207);
const MINI_DISPLAY_SIZE: (u32, u32) = (86, 86);
const MINI_DISPLAY_POS:  (i32, i32) = (10, 10);
const TOP_DETAILS_PADDING: (u32, u32) = (10, 0);
const MENU_MARGIN: i32 = 2;
const MENU_ITEM_H_PADDING: u32 = 4;
const MENU_ITEM_HEIGHT: u32 = 18;
const SHORT_PATH_CHARS: usize = 2;
const SCROLLBAR_WIDTH: u32 = 5;

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
  //canvas.set_clip_rect(Rect::from(position));
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
  //canvas.set_clip_rect(None);
  click && (index as isize == cursor)
}

#[derive(Clone)]
enum MenuLocation {
  Main,
  Options,
  PalettePicker,
  ScalePicker,
  AskForRestart,
  FileExplorer {
    path: PathBuf,
    items: Vec<PathBuf>
  },
}
impl MenuLocation {
  pub fn friendly_name(&self) -> &'static str {
    match self {
      Self::Main => "Main menu",
      Self::Options => "Options",
      Self::PalettePicker => "Color palette",
      Self::ScalePicker => "Display scale",
      Self::AskForRestart => "Restart",
      Self::FileExplorer { .. } => "File explorer"
    }
  }
}

pub struct Menu {
  active: bool,
  activation_anim_state: Animatable,
  cursor: isize,
  menu_stack: Vec<MenuLocation>,
  scroll: i32,
  clicked: bool,
  has_game: bool,
}
impl Menu {
  pub fn new() -> Self {
    Self {
      active: false,
      activation_anim_state: Animatable::new(),
      cursor: 0,
      menu_stack: vec![MenuLocation::Main],
      scroll: 0,
      clicked: false,
      has_game: false,
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
      self.menu_prepare_for_navigation();
      self.menu_stack = vec![MenuLocation::Main];
    }
    self.activation_anim_state.target = (active as u32) as f32;
    self.active = active;
  }
  pub fn skip_activation_animation(&mut self) {
    self.activation_anim_state.value = self.activation_anim_state.target;
    self.activation_anim_state.step();
  }
  ///Process events
  pub fn process_evt(&mut self, event: &Event) {
    match event {
      Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } if self.has_game => {
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
  pub fn always_update(
    &mut self,
    gb: &Gameboy,
  ) {
    //check if game is loaded
    self.has_game = gb.get_mbc_name() != "N/A";
  }

  fn menu_prepare_for_navigation(&mut self) {
    self.clicked = false;
    self.cursor = 0;
    self.scroll = 0;
  }

  fn menu_go_back(&mut self) {
    self.menu_prepare_for_navigation();
    self.menu_stack.pop();
  }

  fn menu_goto(&mut self, to: MenuLocation) {
    self.menu_prepare_for_navigation();
    self.menu_stack.push(to);
  }

  fn file_explorer_goto(&mut self, path: PathBuf) {
    //TODO: This is bad, very bad
    let items = fs::read_dir(&path).unwrap().map(|x| x.unwrap().path()).collect();
    if matches!(self.menu_stack.last().unwrap(), MenuLocation::FileExplorer { .. }) {
      self.menu_go_back()
    }
    self.menu_goto(MenuLocation::FileExplorer { path, items });
  }

  fn file_explorer_goto_home(&mut self) {
    self.file_explorer_goto(dirs::home_dir().unwrap());
  }

  fn file_explorer_open(&mut self, path: PathBuf) {
    let metadata = fs::metadata(&path).unwrap();
    if metadata.is_file() {
      println!("[INFO] open file {}", path.to_str().unwrap());
    } else {
      self.file_explorer_goto(path);
    }
  }

  pub fn update(
    &mut self,
    canvas: &mut Canvas<Window>,
    gb: &mut Gameboy,
    gb_texture: &Texture,
    text: &mut TextRenderer,
    config: &mut Configuration,
    do_exit: &mut bool,
  ) {
    //Get canvas resoultion
    let res = canvas.output_size().unwrap(); //HACK: should use logical size, but it returns 0,0

    //Update avtivation animation
    self.activation_anim_state.step();

    //Clear canvas
    canvas.set_draw_color(BACKGROUND_COLOR);
    canvas.clear();

    //check for small screen
    let small =  config.scale.scale_or_default() == 1;

    //spcial activation animation for 1x
    if small {
      let alpha = (self.activation_anim_state.value * 255.) as u8;
      if alpha != 255 {
        canvas.copy(gb_texture, None, None).unwrap();
        canvas.set_draw_color(Color::RGBA(BACKGROUND_COLOR.r, BACKGROUND_COLOR.g, BACKGROUND_COLOR.b, alpha));
        canvas.fill_rect(Rect::from((0, 0, res.0, res.1))).unwrap();
      }
    }

    //top details
    if !small {
      const ANIM_DIST: f32 = 25.;
      let opa = (self.activation_anim_state.value * 255.) as u32 as u8;
      let x_anim_offset = ANIM_DIST - (self.activation_anim_state.value * ANIM_DIST);
      let x_pos = MINI_DISPLAY_POS.0 + MINI_DISPLAY_SIZE.0 as i32 + TOP_DETAILS_PADDING.0 as i32;
      let y_pos = MINI_DISPLAY_POS.1 + TOP_DETAILS_PADDING.1 as i32;
      let rom_header = gb.get_rom_header();
      //Get the game title
      let display_name = if self.has_game {
        rom_header.name.as_str()
      } else {
        "No game"
      };
      //Compute game title text size to fit the width
      let computed_scale = ((res.0 as f32 - x_pos as f32) / (display_name.len() as f32 * text.char_size(1.).0 as f32)).min(2.);
      
      //Game title text
      text.set_color(Color::RGBA(0, 0, 0, opa));
      text.render(
        canvas, 
        (x_pos + x_anim_offset as i32, y_pos), 
        computed_scale,
        display_name
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
        if self.has_game {
          "Paused"
        } else {
          "Please load a Gameboy ROM"
        }
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
      let list_start_y_noscroll = if !small {
        //TODO animation for 2x+
        (MINI_DISPLAY_POS.1 << 1) + MINI_DISPLAY_SIZE.1 as i32
      } else {
        //has activation animation for 1x
        MENU_MARGIN + res.1 as i32 - (res.1 as f32 * self.activation_anim_state.value) as i32
      }; 
      let list_start_y = list_start_y_noscroll - self.scroll;
      //Macros to display menu items conviniently
      //THIS IS A HUGE HACK AND I WENT ***TOO*** FAR WITH THESE MACROS!!!
      //BUT HEY IF IT WORKS IT WORKS
      let mut x_position: (i32, i32, u32, u32) = (0, list_start_y, res.0, MENU_ITEM_HEIGHT);
      let mut x_index = 0;
      let mut x_cursor_y: Option<i32> = None;
      macro_rules! define_menu_item {
        ($text: expr, $on_click: block) => {{
          if self.cursor == x_index {
            x_cursor_y = Some(x_position.1);
          }
          if menu_item($text, x_position, canvas, text, self.cursor, x_index as usize, self.clicked) {
            $on_click;
          }
          x_position.1 += x_position.3 as i32 + MENU_MARGIN;
          x_index += 1;
          let _ = x_index;
        };};
        ($text: expr, $target: expr) => {{
          define_menu_item!($text, {
            self.menu_goto($target);
          });
        };};
        ($text: expr) => {{
          define_menu_item!($text, {});
        }};
      }
      macro_rules! add_spacing {
        ($pixels: expr) => {{
          let pixels: i32 = $pixels;
          x_position.1 += pixels;
        };};
      }
      macro_rules! define_radio_group {
        ($rg_value_mut_ref: expr, $rg_block: block) => {{
          {
            let x_radio: &mut _ = $rg_value_mut_ref;
            macro_rules! define_radio_item {
              ($ri_text: expr, $ri_pattern: pat, $ri_value: expr, $ri_on_click: block) => {{
                define_menu_item!(&format!("{} {}", if matches!(*x_radio, $ri_pattern) { ">" } else { " " }, $ri_text), {
                  *x_radio = ($ri_value);
                  $ri_on_click
                });
              };};
              ($ri_text: expr, $ri_pattern: pat, $ri_value: expr) => {{
                define_radio_item!($ri_text, $ri_pattern, $ri_value, {});
              };};
            }
            $rg_block
          }
        };};
      }

      //Set clip before rendering the menu
      canvas.set_clip_rect(Rect::from((0, list_start_y_noscroll, res.0, res.1)));

      //Get top item from the menu stack
      let top_item = self.menu_stack.last().unwrap().clone();

      //If menu stack contains more then 1 item allow going back
      if self.menu_stack.len() > 1 {
        define_menu_item!("Back", { self.menu_go_back() });
        add_spacing!(3);
      }

      //Menu layouts
      match top_item {
        MenuLocation::Main => {
          if self.has_game {
            define_menu_item!("Resume", {
              self.set_activated_state(false);
            });
          }
          define_menu_item!("Load ROM file...", {
            match config.last_path.clone() {
              Some(x) => self.file_explorer_goto(x),
              None => self.file_explorer_goto_home()
            }
          });
          define_menu_item!("Options...", MenuLocation::Options);
          define_menu_item!("Exit", { *do_exit = true });
        },
        MenuLocation::Options => {
          define_menu_item!("Color palette...", MenuLocation::PalettePicker);
          define_menu_item!("Display scale...", MenuLocation::ScalePicker);
        },
        MenuLocation::PalettePicker => {
          define_radio_group!(&mut config.palette, {
            define_radio_item!(Palette::Grayscale.get_name(), Palette::Grayscale, Palette::Grayscale);
            define_radio_item!(Palette::Green.get_name(), Palette::Green, Palette::Green);
          });
        }
        MenuLocation::ScalePicker => {
          let mut needs_size_change = false;
          define_radio_group!(&mut config.scale, {
            define_radio_item!("1x (unsupported)", WindowScale::Scale(1), WindowScale::Scale(1), { needs_size_change = true });
            define_radio_item!("2x (recommended)", WindowScale::Scale(2), WindowScale::Scale(2), { needs_size_change = true });
            define_radio_item!("3x", WindowScale::Scale(3), WindowScale::Scale(3), { needs_size_change = true });
            define_radio_item!("4x", WindowScale::Scale(4), WindowScale::Scale(4), { needs_size_change = true });
            define_radio_item!("5x", WindowScale::Scale(5), WindowScale::Scale(5), { needs_size_change = true });
            define_radio_item!("Maximized", WindowScale::Maximized, WindowScale::Maximized, { needs_size_change = true });
            define_radio_item!("Fullscreen", WindowScale::Fullscreen, WindowScale::Fullscreen, { needs_size_change = true });
          });
          if needs_size_change {
            match config.scale {
              WindowScale::Scale(scale) => {
                canvas.window_mut().restore();
                canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap();
                canvas.window_mut().set_size(scale * GB_WIDTH as u32, scale * GB_HEIGHT as u32).unwrap();
              },
              WindowScale::Maximized => {
                canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap();
                canvas.window_mut().maximize();
              },
              WindowScale::Fullscreen => {
                canvas.window_mut().set_fullscreen(FullscreenType::Desktop).unwrap();
              },
            }
          }
        }
        MenuLocation::AskForRestart => {
          define_menu_item!("Restart now to apply changes", { *do_exit = true });
        }
        MenuLocation::FileExplorer { items, path } => {
          define_menu_item!("Home", {
            self.file_explorer_goto_home();
            config.last_path = None;
          });
          add_spacing!(3);
          if let Some(parent) = path.parent() {
            define_menu_item!("..", {
              self.file_explorer_goto(parent.to_owned());
              config.last_path = Some(path.clone());
            });
          }
          if !items.is_empty() {
            for item in items {
              define_menu_item!(item.file_name().unwrap().to_str().unwrap(), {
                self.file_explorer_open(item);
                config.last_path = Some(path.clone());
              });
            }
          } else {
            define_menu_item!("This directory is empty");
          }
        }
      }

      // Unset clip rect
      canvas.set_clip_rect(None);

      // Update scroll
      //TODO rewrite!!
      if let Some(cursor_y) = x_cursor_y {
        let cursor_underscroll = cursor_y - (res.1 as i32 - (text.char_size(1.).1 as i32 + 2) - 2 * MENU_ITEM_HEIGHT as i32);
        let cursor_overscroll = cursor_y - list_start_y_noscroll - MENU_ITEM_HEIGHT as i32;
        self.scroll += cursor_underscroll.max(0) + cursor_overscroll.min(0);
      }
      self.scroll = self.scroll.max(0);
      
      // Draw scroll bar
      let viewport_height = res.1 as f32 - list_start_y_noscroll as f32 - (text.char_size(1.).1 as f32 + 2.);
      let scrollbar_visible = (x_position.1 - list_start_y) as f32 > viewport_height;
      if scrollbar_visible {
        //Compute stuff
        let progress: f32 = self.cursor as f32 / (x_index - 1) as f32;
        let viewport_height_ratio: f32 = (viewport_height / (MENU_ITEM_HEIGHT + MENU_MARGIN as u32) as f32) / (x_index - 1) as f32;
        //Use that stuff to compute scrollbar pos
        let scrollbar_h = viewport_height_ratio * viewport_height;
        let y_correction: f32 = - (scrollbar_h * progress);
        let scrollbar_rect = Rect::from((
          res.0 as i32 - SCROLLBAR_WIDTH as i32,
          list_start_y_noscroll + ((progress * viewport_height) + y_correction) as i32,
          SCROLLBAR_WIDTH,
          scrollbar_h as u32
        ));
        let scrollbar_bg_rect = Rect::from((
          res.0 as i32 - SCROLLBAR_WIDTH as i32,
          list_start_y_noscroll,
          SCROLLBAR_WIDTH,
          viewport_height as u32
        ));
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 96));
        canvas.fill_rects(&[scrollbar_rect, scrollbar_rect, scrollbar_bg_rect]).unwrap();
      }

      // Limit cursor
      if self.cursor < 0 {
        self.cursor = x_index - 1;
      } else if self.cursor >= x_index {
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
      text.render(canvas, (3, text_y), 1., if !small {
        "\x1e\x1f Move cursor \x04 Confirm"
      } else {
        "\x1e\x1f Move \x04 OK"
      });
      //version text
      let ver_x = (res.0 - text.text_size(CRATE_VERSION, 1.).0 - 3) as i32;
      text.set_color(Color::RGBA(255, 255, 255, opa / 3));
      text.render(canvas, (ver_x, text_y), 1., CRATE_VERSION);
    }

    //Draw display
    if !small {
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

    //Reset clicked state
    self.clicked = false;
  }
}
impl Default for Menu {
  fn default() -> Self { Self::new() }
}
