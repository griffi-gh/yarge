use std::{path::PathBuf, fs};
use serde::{Serialize, Deserialize};
use crate::data_dir::DataDir;

const CONFIG_FILE_NAME: &str = "options.bin";

#[derive(Default, Serialize, Deserialize, Clone, Copy)]
pub enum Palette {
  #[default]
  Grayscale,
  Green,
  Custom([u32; 4])
}
impl Palette {
  pub fn get_map(&self) -> [u32; 4] {
    match self {
      Self::Grayscale => [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x0000000],
      Self::Green     => [0x00e0f8d0, 0x0088c070, 0x00346856, 0x0081820],
      Self::Custom(x) => *x
    }
  }
  pub fn get_name(&self) -> &'static str {
    match self {
      Self::Grayscale => "Grayscale",
      Self::Green     => "Green",
      Self::Custom(_) => "Custom"
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

#[derive(Serialize, Deserialize)]
pub struct Configuration {
  pub palette: Palette,
  pub framerate: FramerateLimit,
  pub scale: WindowScale,
  pub last_rom: Option<PathBuf>,
  pub last_path: Option<PathBuf>,
  pub closed_properly: bool,
}
impl Default for Configuration {
  fn default() -> Self {
    Self {
      palette: Default::default(),
      framerate: Default::default(),
      scale: Default::default(),
      last_rom: Default::default(),
      last_path: Default::default(),
      closed_properly: true
    }
  }
}
impl Configuration {
  pub fn save(&self) -> anyhow::Result<()> {
    DataDir::ensure_exists()?;
    let mut path = DataDir::get_path();
    path.push(CONFIG_FILE_NAME);
    fs::write(path, bincode::serialize(self)?)?;
    Ok(())
  }
  pub fn load() -> anyhow::Result<Self> {
    let mut path = DataDir::get_path();
    path.push(CONFIG_FILE_NAME);
    Ok(bincode::deserialize(&fs::read(path)?)?)
  }
  pub fn load_or_default() -> Self {
    Self::load().map_err(|_| {
      println!("[WARN] Failed to load configuration");
    }).unwrap_or_default()
  }
}
