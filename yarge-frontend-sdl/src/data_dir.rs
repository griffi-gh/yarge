use std::{path::PathBuf, fs};
use anyhow::Result;

const YARGE_DIR_NAME: &str = "yarge-sdl";

pub struct DataDir;
impl DataDir {
  pub fn get_path() -> PathBuf {
    let mut dir = {
      #[cfg(feature = "global_config")] {
        dirs::config_dir().unwrap_or_else(|| PathBuf::from("."))
      }
      #[cfg(not(feature = "global_config"))] {
        PathBuf::from(".")
      }
    };
    dir.push(YARGE_DIR_NAME);
    dir
  }

  pub fn ensure_exists() -> anyhow::Result<()> {
    let path = Self::get_path();
    #[cfg(feature = "global_config")] {
      fs::create_dir_all(&path)?;
    }
    #[cfg(not(feature = "global_config"))] {
      fs::create_dir(&path)?;
    }
    Ok(())
  }
}
