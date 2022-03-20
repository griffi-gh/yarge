pub(crate) mod gb;
use gb::{Gameboy, GameboyBuilder};
use std::sync::{Arc, Mutex};
use clap::Parser;

#[cfg(feature = "gui")]
mod gui;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
  #[clap(short, long)]
  skip_bootrom: bool,
  #[clap(short, long)]
  gui: bool,
  path: String
}

fn main() {
  let args = Args::parse();
  let rom_path = &args.path[..];
  let gb = GameboyBuilder::new()
    .init(true)
    .skip_bootrom(args.skip_bootrom)
    .load_rom_file(rom_path).expect("Failed to load the ROM file")
    .build();
  let gb = Arc::new(Mutex::new(gb));
  let gb_thread = Gameboy::run_thread(&gb);
  if !args.gui {
    #[cfg(feature = "gui")]
    println!("Hint: Use --gui or -g to enable the GUI");
    gb_thread.join().unwrap();
  } else {
    #[cfg(not(feature = "gui"))]
    panic!("Build with the 'gui' feature enabled to use the --gui flag");
    #[cfg(feature = "gui")] {
      gui::GuiState::new(Arc::clone(&gb)).init();
    }
  }
}
