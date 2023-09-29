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
  path::PathBuf,
  fs
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
  config::{Configuration, Palette, WindowScale}, FAT_TEXTURE, saves::SaveManager
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

#[allow(clippy::too_many_arguments)]
fn menu_item(
  text: &str,
  position: (i32, i32, u32, u32),
  dpi_scale: f32,
  canvas: &mut Canvas<Window>,
  text_renderer: &mut TextRenderer,
  cursor: isize,
  index: usize,
  click: bool,
) -> bool {
  let v_padding = (position.3 as i32 - text_renderer.char_size(1.).1 as i32).max(0) as u32 / 2;
  //canvas.set_clip_rect(Rect::from(position));
  if index as isize == cursor {
    let position_dpi = Rect::from((
      ((position.0 as f32) * dpi_scale) as i32,
      ((position.1 as f32) * dpi_scale) as i32,
      ((position.2 as f32) * dpi_scale) as u32,
      ((position.3 as f32) * dpi_scale) as u32,
    ));
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 96));
    canvas.fill_rect(position_dpi).unwrap();
    canvas.set_draw_color(Color::RGBA(0, 0, 0, 128));
    canvas.draw_rect(position_dpi).unwrap();
    text_renderer.set_color(Color::RGBA(255, 255, 255, 255));
  } else {
    text_renderer.set_color(Color::RGBA(0, 0, 0, 255));
  }
  text_renderer.render(canvas, (
    ((position.0 + MENU_ITEM_H_PADDING as i32) as f32 * dpi_scale) as i32,
    ((position.1 + v_padding as i32) as f32 * dpi_scale) as i32
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
  SpeedPicker,
  SaveSlotPicker,
  SaveSlotConfirm(u8),
  AskForRestart,
  FileExplorer {
    path: PathBuf,
    items: Vec<PathBuf>
  },
  ClosedImproperly,
}
impl MenuLocation {
  pub fn friendly_name(&self) -> &'static str {
    match self {
      Self::Main => "Main menu",
      Self::Options => "Options",
      Self::PalettePicker => "Color palette",
      Self::ScalePicker => "Display scale",
      Self::SpeedPicker => "Speed",
      Self::SaveSlotPicker => "Save slot",
      Self::SaveSlotConfirm(_) => "Confirm restart",
      Self::AskForRestart => "Restart",
      Self::FileExplorer { .. } => "File explorer",
      Self::ClosedImproperly => "Warning",
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
  schedule_save: bool,
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
      schedule_save: false,
    }
  }
  pub fn is_active(&self) -> bool {
    self.active
  }
  pub fn is_visible(&self) -> bool {
    self.is_active() || self.activation_anim_state.is_animating()
  }
  pub fn set_activated_state(&mut self, active: bool) {
    if self.active != active {
      self.on_activation_change(active);
    }
    if (!self.is_visible()) && (!self.active) && active {
      //reset menu
      self.menu_prepare_for_navigation();
      self.menu_stack = vec![MenuLocation::Main];
    }
    self.activation_anim_state.target = (active as u32) as f32;
    self.active = active;
  }
  fn on_activation_change(&mut self, state: bool) {
    if state {
      //Save every time menu is opened
      self.schedule_save = true;
    }
  }
  pub fn closed_improperly(&mut self) {
    self.set_activated_state(true);
    self.menu_goto(MenuLocation::ClosedImproperly);
    self.cursor = 1;
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
    println!("Going home");
    self.file_explorer_goto(dirs::home_dir().unwrap());
  }

  fn file_explorer_open(&mut self, path: PathBuf, gb: &mut Gameboy, config: &Configuration) -> bool {
    let metadata = fs::metadata(&path).unwrap();
    if metadata.is_file() {
      println!("[INFO] open file {}", path.to_str().unwrap());
      self.load_file(path, gb, config);
      self.set_activated_state(false);
      false
    } else {
      self.file_explorer_goto(path);
      true
    }
  }

  fn load_file(&mut self, path: PathBuf, gb: &mut Gameboy, config: &Configuration) {
    let data = fs::read(path).unwrap();
    gb.reset();
    gb.load_rom(&data[..]).unwrap();
    SaveManager::load_idk(gb, config.save_slot);
    SaveManager::save(gb, config.save_slot).unwrap(); //Create save file
  }

  pub fn reset_game(
    &mut self,
    gb_texture: &mut Texture,
    gb: &mut Gameboy
  ) {
    self.set_activated_state(true);
    //self.skip_activation_animation();
    gb.reset();
    gb_texture.update(None, FAT_TEXTURE, 3 * GB_WIDTH).unwrap();
    self.has_game = false;
    self.cursor = 0;
  }

  pub fn reboot_game(
    &mut self,
    config: &Configuration,
    gb: &mut Gameboy,
  ) {
    if let Some(path) = &config.last_rom {
      self.load_file(path.into(), gb, config);
    } else {
      gb.reset();
      self.has_game = false;
    }
  }

  #[allow(clippy::too_many_arguments)]
  pub fn update(
    &mut self,
    canvas: &mut Canvas<Window>,
    dpi_scale: f32,
    gb: &mut Gameboy,
    gb_texture: &mut Texture,
    text: &mut TextRenderer,
    config: &mut Configuration,
    do_exit: &mut bool,
  ) {
    fn m (x: i32, y: f32) -> i32 { ((x as f32) * y) as i32 }
    fn mu(x: u32, y: f32) -> u32 { ((x as f32) * y) as u32 }

    //If save is scheduled, do it now
    if self.schedule_save {
      SaveManager::save(gb, config.save_slot).unwrap();
      self.schedule_save = false;
    }

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
      //XXX: this does not handle hidpi properly
      let computed_scale = ((res.0 as f32 - x_pos as f32) / (display_name.len() as f32 * (text.char_size(1.).0 as f32))).min(2.);
      
      //Game title text
      text.set_color(Color::RGBA(0, 0, 0, opa));
      text.render(
        canvas,
        (m(x_pos + x_anim_offset as i32, dpi_scale), m(y_pos, dpi_scale)),
        computed_scale,
        display_name
      );

      //"Paused" text
      text.set_color(Color::RGBA(64, 64, 64, opa));
      text.render(
        canvas,
        (
          m(x_pos + (2. * x_anim_offset) as i32, dpi_scale),
          m(y_pos + text.char_size(2.).1 as i32, dpi_scale)
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
          m(x_pos + (2. * x_anim_offset) as i32, dpi_scale),
          m(MINI_DISPLAY_POS.1 + MINI_DISPLAY_SIZE.1 as i32 - (text.char_size(1.).1 as i32) - TOP_DETAILS_PADDING.1 as i32, dpi_scale)
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
          if menu_item($text, x_position, dpi_scale, canvas, text, self.cursor, x_index as usize, self.clicked) {
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
        ($rg_value_mut_ref: expr, $rg_block: block, $rg_on_any_clicked: block) => {
          {
            let x_radio: &mut _ = $rg_value_mut_ref;
            let mut x_selection_did_change = false;
            macro_rules! define_radio_item {
              ($ri_text: expr, $ri_pattern: pat, $ri_value: expr, $ri_on_click: block) => {{
                define_menu_item!(&format!("{} {}", if matches!(*x_radio, $ri_pattern) { ">" } else { " " }, $ri_text), {
                  *x_radio = ($ri_value);
                  x_selection_did_change = true;
                  $rg_on_any_clicked
                  $ri_on_click
                });
              };};
              ($ri_text: expr, $ri_pattern: pat, $ri_value: expr) => {{
                define_radio_item!($ri_text, $ri_pattern, $ri_value, {});
              };};
            }
            $rg_block
            x_selection_did_change
          }
        };
        ($rg_value_mut_ref: expr, $rg_block: block) => {
          define_radio_group!($rg_value_mut_ref, $rg_block, {})
        }
      }
      macro_rules! define_checkbox {
        ($cb_text: expr, $cb_bool_value: expr, $on_change: block) => {
          {
            let bref: &mut bool = ($cb_bool_value).into();
            let btext: &str = ($cb_text).into();
            define_menu_item!(&format!("[{}] {}", if *bref { "X" } else { " " }, btext), {
              *bref ^= true;
              $on_change;
            });
          }
        };
        ($cb_text: expr, $cb_bool_value: expr) => {
          define_checkbox!($cb_text, $cb_bool_value, {});
        }
      }

      //Set clip before rendering the menu
      canvas.set_clip_rect(Rect::from((0, m(list_start_y_noscroll, dpi_scale), mu(res.0, dpi_scale), mu(res.1, dpi_scale))));

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
            define_menu_item!("Stop", {
              self.reset_game(gb_texture, gb);
            });
            define_menu_item!("Reset", {
              self.reboot_game(config, gb);
              self.set_activated_state(false);
            });
          } else if let Some(resume_path) = &config.last_rom {
            let resume_file_name = resume_path.file_name().map(|x| x.to_str().unwrap_or("")).unwrap_or("");
            define_menu_item!(&format!("Resume {}", resume_file_name), {
              println!("[INFO] resume to path: {}", resume_path.to_str().unwrap());
              self.load_file(resume_path.clone(), gb, config);
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
          define_menu_item!("Speed...", MenuLocation::SpeedPicker);
          define_menu_item!("Save slot...", MenuLocation::SaveSlotPicker);
          define_menu_item!("Exit", { *do_exit = true });
        },
        MenuLocation::Options => {
          define_menu_item!("Color palette...", MenuLocation::PalettePicker);
          define_menu_item!("Display scale...", MenuLocation::ScalePicker);
        },
        MenuLocation::PalettePicker => {
          if define_radio_group!(&mut config.palette, {
            define_radio_item!(Palette::Grayscale.get_name(), Palette::Grayscale, Palette::Grayscale);
            define_radio_item!(Palette::Green.get_name(), Palette::Green, Palette::Green);
          }) {
            config.save_dirty().unwrap();
          }
        }
        MenuLocation::ScalePicker => {
          define_checkbox!(
            if small { "HiDPI Scaling" } else { "HiDPI Scaling (experimental)" }, 
            &mut config.dpi_scaling, 
            { config.save_dirty().unwrap() }
          );
          if config.dpi_scaling {
            define_checkbox!(
              if small { "Allow fract." } else { "HiDPI Scaling: Allow fractional" }, 
              &mut config.dpi_scaling_frac, 
              { config.save_dirty().unwrap() }
            );
          }

          if define_radio_group!(&mut config.scale, {
            define_radio_item!("1x", WindowScale::Scale(1), WindowScale::Scale(1));
            define_radio_item!("2x (recommended)", WindowScale::Scale(2), WindowScale::Scale(2));
            define_radio_item!("3x", WindowScale::Scale(3), WindowScale::Scale(3));
            define_radio_item!("4x", WindowScale::Scale(4), WindowScale::Scale(4));
            define_radio_item!("5x", WindowScale::Scale(5), WindowScale::Scale(5));
            define_radio_item!("Maximized", WindowScale::Maximized, WindowScale::Maximized);
            define_radio_item!("Fullscreen", WindowScale::Fullscreen, WindowScale::Fullscreen);
          }) {
            config.save_dirty().unwrap();
            match config.scale {
              WindowScale::Scale(scale) => {
                canvas.window_mut().restore();
                canvas.window_mut().set_fullscreen(FullscreenType::Off).unwrap();
                canvas.window_mut().set_size(mu(scale * GB_WIDTH as u32, dpi_scale), mu(scale * GB_HEIGHT as u32, dpi_scale)).unwrap();
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
        MenuLocation::SpeedPicker => {
          if define_radio_group!(&mut config.speed, {
            define_radio_item!("1x", 1, 1);
            define_radio_item!("2x", 2, 2);
            define_radio_item!("3x", 3, 3);
            define_radio_item!("4x", 4, 4);
            define_radio_item!("5x", 5, 5);
            define_radio_item!("6x", 6, 6);
            define_radio_item!("7x", 7, 7);
            define_radio_item!("8x", 8, 8);
            define_radio_item!("9x", 9, 9);
            define_radio_item!("10x", 10, 10);
          }) {
            config.save_dirty().unwrap();
          }
        }
        MenuLocation::SaveSlotPicker => {
          let mut save_slot = config.save_slot;
          if define_radio_group!(&mut save_slot, {
            define_radio_item!("Slot 1", 0, 0);
            define_radio_item!("Slot 2", 1, 1);
            define_radio_item!("Slot 3", 2, 2);
            define_radio_item!("Slot 4", 3, 3);
            define_radio_item!("Slot 5", 4, 4);
          }) && (save_slot != config.save_slot) {
            if self.has_game {
              self.menu_goto(MenuLocation::SaveSlotConfirm(save_slot));
              self.cursor = 1;
            } else {
              config.save_slot = save_slot;
              config.save_dirty().unwrap();
            }
          }
        }
        MenuLocation::SaveSlotConfirm(save_slot) => {
          define_menu_item!(if small { "Restart" } else { "Restart the game now?" }, {
            SaveManager::save(gb, config.save_slot).unwrap();
            config.save_slot = save_slot;
            config.save_dirty().unwrap();
            self.reboot_game(config, gb); //calls load internally
            SaveManager::save(gb, config.save_slot).unwrap();
            self.set_activated_state(false);
          });
        }
        MenuLocation::AskForRestart => {
          define_menu_item!("Restart now to apply changes", { *do_exit = true });
        }
        MenuLocation::FileExplorer { items, path } => {
          define_menu_item!("Home", {
            self.file_explorer_goto_home();
            config.last_path = None;
            config.save_dirty().unwrap();
          });
          add_spacing!(3);
          if let Some(parent) = path.parent() {
            define_menu_item!("..", {
              self.file_explorer_goto(parent.to_owned());
              config.last_path = Some(parent.to_owned());
              config.save_dirty().unwrap();
            });
          }
          if !items.is_empty() {
            for item in items {
              define_menu_item!(item.file_name().unwrap().to_str().unwrap(), {
                if self.file_explorer_open(item.clone(), gb, config) {
                  config.last_path = Some(item.clone());
                } else {
                  config.last_rom = Some(item.clone());
                }
                config.save_dirty().unwrap();
              });
            }
          } else {
            define_menu_item!("This directory is empty");
          }
        }
        MenuLocation::ClosedImproperly => {
          define_menu_item!("Continue", {
            if self.has_game {
              self.set_activated_state(false);
            } else {
              self.menu_go_back();
            }
          });
          add_spacing!(5);
          if !small {
            define_menu_item!("Yarge didn't close correcly");
            define_menu_item!("Some data may be lost");
          } else {
            define_menu_item!("Yarge didn't");
            define_menu_item!("close correcly");
            define_menu_item!("Some data may");
            define_menu_item!("be lost");
          }
        }
      }

      // Unset clip rect
      canvas.set_clip_rect(None);

      // Update scroll
      //TODO rewrite!!
      if let Some(cursor_y) = x_cursor_y {
        let cursor_underscroll = cursor_y - ((res.1 as f32 / dpi_scale) as i32 - (text.char_size(1.).1 as i32 + 2) - 2 * MENU_ITEM_HEIGHT as i32);
        let cursor_overscroll = cursor_y - list_start_y_noscroll - MENU_ITEM_HEIGHT as i32;
        self.scroll += cursor_underscroll.max(0) + cursor_overscroll.min(0);
      }
      self.scroll = self.scroll.max(0);
      
      // Draw scroll bar
      let viewport_height = (res.1 as f32 / dpi_scale) - list_start_y_noscroll as f32 - (text.char_size(1.).1 as f32 + 2.);
      let scrollbar_visible = (x_position.1 - list_start_y) as f32 > viewport_height;
      if scrollbar_visible {
        //Compute stuff
        let progress: f32 = self.cursor as f32 / (x_index - 1) as f32;
        let viewport_height_ratio: f32 = (viewport_height / (MENU_ITEM_HEIGHT + MENU_MARGIN as u32) as f32) / (x_index - 1) as f32;
        //Use that stuff to compute scrollbar pos
        let scrollbar_h = viewport_height_ratio * viewport_height;
        let y_correction: f32 = - (scrollbar_h * progress);
        let scrollbar_rect = Rect::from((
          res.0 as i32 - m(SCROLLBAR_WIDTH as i32, dpi_scale),
          m(list_start_y_noscroll + ((progress * viewport_height) + y_correction) as i32, dpi_scale),
          mu(SCROLLBAR_WIDTH, dpi_scale),
          mu(scrollbar_h as u32, dpi_scale)
        ));
        let scrollbar_bg_rect = Rect::from((
          res.0 as i32 - m(SCROLLBAR_WIDTH as i32, dpi_scale),
          m(list_start_y_noscroll, dpi_scale),
          mu(SCROLLBAR_WIDTH, dpi_scale),
          mu(viewport_height as u32, dpi_scale)
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
      let offst = -(h as i32 - (self.activation_anim_state.value * h as f32) as i32);
      //box
      canvas.set_draw_color(Color::RGBA(0, 0, 0, opa / 4));
      canvas.fill_rects(&[
        Rect::from((0, res.1 as i32 - m(h as i32 + offst, dpi_scale) - 1, res.0, mu(h, dpi_scale) + 1)),
        Rect::from((0, res.1 as i32 - m(h as i32 + offst, dpi_scale), res.0, mu(h, dpi_scale)))
      ]).unwrap();
      //compute y coord
      let text_y = res.1 as i32 - m(h as i32 - 1 + offst, dpi_scale);
      //help text
      text.set_color(Color::RGBA(255, 255, 255, opa));
      text.render(canvas, (m(3, dpi_scale), text_y), 1., if !small {
        "\x1e\x1f Move cursor \x04 Confirm"
      } else {
        "\x1e\x1f Move \x04 OK"
      });
      //version text
      let ver_x = res.0 as i32 - m((text.text_size(CRATE_VERSION, 1.).0 + 3) as i32, dpi_scale);
      text.set_color(Color::RGBA(255, 255, 255, opa / 3));
      text.render(canvas, (ver_x, text_y), 1., CRATE_VERSION);
    }

    //Draw display
    if !small {
      let anim = self.activation_anim_state.value;
      let display_pos = (
        (anim * (MINI_DISPLAY_POS.0 as f32 * dpi_scale)) as i32,
        (anim * (MINI_DISPLAY_POS.1 as f32 * dpi_scale)) as i32,
        res.0 - (anim * (res.0 as f32 - (MINI_DISPLAY_SIZE.0 as f32 * dpi_scale))) as u32,
        res.1 - (anim * (res.1 as f32 - (MINI_DISPLAY_SIZE.1 as f32 * dpi_scale))) as u32,
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
