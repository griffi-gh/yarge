use std::{path::PathBuf, fs};
use serde::{Serialize, Deserialize};
use crate::data_dir::DataDir;

#[derive(Default, Serialize, Deserialize)]
pub enum Palette {
  #[default]
  Grayscale,
  Bgb,
  Custom([u32; 4])
}

#[derive(Default, Serialize, Deserialize)]
pub enum FramerateLimit {
  #[default]
  VSync,
  Limit(u32),
  Unlimited,
}

#[derive(Serialize, Deserialize)]
pub enum WindowScale {
  Scale(u32),
  Fullscreen
}
impl Default for WindowScale {
  fn default() -> Self { Self::Scale(2) }
}


const YARGE_CONFIG_FILE_NAME: &str = "options.bin";

#[derive(Default, Serialize, Deserialize)]
pub struct Configuration {
  pub palette: Palette,
  pub framerate: FramerateLimit,
  pub scale: WindowScale
}
impl Configuration {
  pub fn save(&self) -> anyhow::Result<()> {
    DataDir::ensure_exists();
    let path = DataDir::get_path();
    fs::write(path, toml::to_vec(self)?)?;
    Ok(())
  }
  pub fn load() -> anyhow::Result<Self> {
    let path = DataDir::get_path();
    Ok(toml::from_slice(&fs::read(path)?)?)
  }
  pub fn load_or_default() -> Self {
    Self::load().unwrap_or_default()
  }
}