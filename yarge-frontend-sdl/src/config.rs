use std::{path::PathBuf, fs};
use sdl2::pixels::Color;
use serde::{Serialize, Deserialize};
use crate::data_dir::DataDir;

const CONFIG_FILE_NAME: &str = "options.bin";

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub enum Palette {
  #[default]
  Grayscale,
  GrayscaleDark,
  Green,
  Custom([u32; 4])
}
impl Palette {
  pub fn get_map(&self) -> [u32; 4] {
    match self {
      Self::Grayscale     => [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x00000000],
      Self::GrayscaleDark => [0x00000000, 0x00555555, 0x00aaaaaa, 0x00ffffff],
      Self::Green         => [0x00e0f8d0, 0x0088c070, 0x00346856, 0x00081820],
      Self::Custom(x) => *x
    }
  }
  pub fn get_name(&self) -> &'static str {
    match self {
      Self::Grayscale => "Grayscale",
      Self::GrayscaleDark => "Grayscale (Dark)",
      Self::Green     => "Green",
      Self::Custom(_) => "Custom"
    }
  }
  ///Should overlay text be white?
  pub fn is_dark(&self) -> bool {
    match self {
      Self::Grayscale => false,
      Self::GrayscaleDark => true,
      Self::Green     => false,
      Self::Custom(_) => false //TODO check color brightness
    }
  }
}

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub enum FramerateLimit {
  #[default]
  VSync,
  Limit(u32),
  Unlimited,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum WindowScale {
  Scale(u32),
  Maximized,
  Fullscreen
}
impl Default for WindowScale {
  fn default() -> Self { Self::Scale(2) }
}
impl WindowScale {
  pub fn scale_or_default(&self) -> u32 {
    if let Self::Scale(scale) = *self {
      scale
    } else {
      2
    }
  }
}

pub struct ThemeColors {
  //Example: Most text
  pub text: Color,
  //Example: "Paused" text, menu stack text
  pub text_faded: Color,
  ///Example: Global menu background
  pub background: Color,
  ///Example: Selected menu item bg, bottom panel bg
  pub caret: Color,
  ///Example: Selected menu item text color, bottom panel text color
  pub caret_text_color: Color,
  ///Example: Selected menu item border
  pub caret_border: Color,
  ///Scrollbar color, relies on alpha, scrollbar is layered on top of it's bg with same color
  pub scrollbar_color: Color,
}

#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug)]
pub enum UiTheme {
  Light,
  Dark,
  #[default] SystemPreference,
}
impl UiTheme {
  pub fn resolve(self) -> Self {
    match self {
      Self::SystemPreference => {
        #[cfg(feature = "system-theme")] {
          use dark_light::Mode;
          match dark_light::detect() {
            Mode::Default | Mode::Light => Self::Light,
            Mode::Dark => Self::Dark,
          }
        }
        #[cfg(not(feature = "system-theme"))] {
          Self::Light
        }
      }
      _ => self
    }
  }

  #[inline]
  pub const fn colors(&self) -> ThemeColors {
    match *self {
      Self::Light => ThemeColors {
        text: Color::BLACK,
        text_faded: Color::RGB(64, 64, 64),
        background: Color::RGB(233, 226, 207),
        caret: Color::RGBA(64, 64, 64, 255 / 2),
        caret_text_color: Color::WHITE,
        caret_border: Color::RGBA(0, 0, 0, 128),
        scrollbar_color: Color::RGBA(0, 0, 0, 96),
      },
      Self::Dark => ThemeColors {
        text: Color::WHITE,
        text_faded: Color::RGB(240, 240, 240),
        background: Color::RGB(0x1e, 0x1e, 0x1e),
        caret: Color::RGBA(255, 255, 255, 255 / 2),
        caret_text_color: Color::BLACK,
        caret_border: Color::RGBA(255, 255, 255, 128),
        scrollbar_color: Color::RGBA(255, 255, 255, 96),
      },
      Self::SystemPreference => unreachable!()
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct FpsOverlayOptions {
  pub enable: bool,
  pub hi_contrast: bool,
  pub smol: bool,
  pub round: bool,
}

impl Default for FpsOverlayOptions {
  fn default() -> Self {
    Self {
      enable: false,
      hi_contrast: true,
      smol: false,
      round: true,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
  pub palette: Palette,
  pub framerate: FramerateLimit,
  pub scale: WindowScale,
  pub last_rom: Option<PathBuf>,
  pub last_path: Option<PathBuf>,
  pub closed_properly: bool,
  pub default_speed: u8,
  pub save_slot: u8,
  pub dpi_scaling: bool,
  pub dpi_scaling_frac: bool,
  pub theme: UiTheme,
  pub fps: FpsOverlayOptions,
}
impl Default for Configuration {
  fn default() -> Self {
    Self {
      palette: Default::default(),
      framerate: Default::default(),
      scale: Default::default(),
      last_rom: Default::default(),
      last_path: Default::default(),
      closed_properly: true,
      default_speed: 1,
      save_slot: 0,
      dpi_scaling: true,
      dpi_scaling_frac: false,
      theme: Default::default(),
      fps: FpsOverlayOptions::default(),
    }
  }
}
impl Configuration {
  fn save(&self) -> anyhow::Result<()> {
    DataDir::ensure_exists()?;
    let mut path = DataDir::get_path();
    path.push(CONFIG_FILE_NAME);
    fs::write(path, bincode::serialize(self)?)?;
    Ok(())
  }
  pub fn save_dirty(&mut self) -> anyhow::Result<()> {
    println!("[CONF/INFO] Saving configuration (dirty)...");
    let original = self.closed_properly;
    self.closed_properly = false;
    self.save()?;
    self.closed_properly = original;
    Ok(())
  }
  pub fn save_clean(mut self) -> anyhow::Result<()> {
    println!("[CONF/INFO] Saving configuration (clean)...");
    self.closed_properly = true;
    self.save()?;
    Ok(())
  }

  pub fn load() -> anyhow::Result<Self> {
    println!("[CONF/INFO] Loading configuration...");
    let mut path = DataDir::get_path();
    path.push(CONFIG_FILE_NAME);
    Ok(bincode::deserialize(&fs::read(path)?)?)
  }
  pub fn load_or_default() -> Self {
    Self::load().map_err(|_| {
      println!("[CONF/WARN] Failed to load configuration");
    }).unwrap_or_default()
  }
}
