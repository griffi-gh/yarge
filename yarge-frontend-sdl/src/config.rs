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

const CONFIG_FILE_NAME: &str = "options.bin";

#[derive(Default, Serialize, Deserialize)]
pub struct Configuration {
  pub palette: Palette,
  pub framerate: FramerateLimit,
  pub scale: WindowScale
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
    Self::load().unwrap_or_default()
  }
}