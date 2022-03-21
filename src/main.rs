pub(crate) use rustish_core as gb;
use gb::{Gameboy, GameboyBuilder};
use std::sync::{Arc, Mutex};
use clap::Parser;
use build_time::build_time_local;

pub(crate) const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME: &str = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");

#[cfg(feature = "gui")]
mod gui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long)]
  skip_bootrom: bool,
  #[clap(short, long)]
  nogui: bool,
  path: String
}

fn main() {
  let args = Args::parse();
  println!(
    "[ {} v.{} (built on {}) ]",
    NAME.unwrap_or("<name?>"),
    VERSION.unwrap_or("<version?>"),
    BUILD_TIME
  );
  let rom_path = &args.path[..];
  let gb = GameboyBuilder::new()
    .init(true)
    .skip_bootrom(args.skip_bootrom)
    .load_rom_file(rom_path).expect("Failed to load the ROM file")
    .build();
  let gb = Arc::new(Mutex::new(gb));
  let gb_thread = Gameboy::run_thread(&gb);
  if args.nogui {
    gb_thread.join().unwrap();
  } else {
    #[cfg(not(feature = "gui"))]
    panic!("No GUI support, use the --nogui (-n) flag or build {} with 'gui' feature", NAME.unwrap_or(""));
    #[cfg(feature = "gui")] {
      gui::GuiState::new(Arc::clone(&gb)).init();
    }
  }
}
