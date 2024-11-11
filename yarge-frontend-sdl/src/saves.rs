use anyhow::Result;
use std::{fs, path::PathBuf};
use yarge_core::Gameboy;
use crate::data_dir::DataDir;

const DISALLOW_FILENAME: &[char] = &[
  ' ', '\n', '\r', '\\', '/',
  '#', '%', '&', '{', '}',
  '<', '>', '*', '?', '$',
  '!', '\'', '"', ':', '@',
  '+', '`', '|', '[', ']',
  ',', '.'
];

pub struct SaveManager;
impl SaveManager {
  fn file_name(name: &str, slot: u8) -> String {
    format!(
      "SAVE{slot}_{name}.sav",
      name = name.replace(DISALLOW_FILENAME, "__").to_ascii_uppercase()
    )
  }

  fn file_path(name: &str, slot: u8) -> PathBuf {
    let mut path = DataDir::get_path();
    let file_name = Self::file_name(name, slot);
    path.push(file_name);
    path
  }

  pub fn save(gb: &Gameboy, slot: u8) -> Result<()> {
    println!("[SAVE/INFO] Writing ERAM save data...");

    if !gb.has_save_data() { return Ok(()) }
    if let Some(data) = gb.get_save_data() {
      if data.is_empty() { return Ok(()) }
      //Resolve save path and write data
      let path = Self::file_path(&gb.get_rom_header().name, slot);
      fs::write(path, data)?;
    }
    Ok(())
  }

  pub fn load(gb: &mut Gameboy, slot: u8) -> Result<()> {
    println!("[SAVE/INFO] Loading ERAM save data...");

    if !gb.has_save_data() { return Ok(()) }

    //Resolve save path and read data
    let path = Self::file_path(&gb.get_rom_header().name, slot);
    let data = fs::read(path)?;
    if data.is_empty() { return Ok(()) }
    
    gb.set_save_data(data);

    Ok(())
  }

  pub fn exists(gb: &Gameboy, slot: u8) -> bool {
    let path = Self::file_path(&gb.get_rom_header().name, slot);
    path.exists()
  }

  pub fn load_idk(gb: &mut Gameboy, slot: u8) {
    if Self::load(gb, slot).is_err() {
      println!("[SAVE/ERR] Failed to load the save file");
    }
  }
}
