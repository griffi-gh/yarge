pub(crate) use rustish_core as gb;
use gb::GameboyBuilder;
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
  path: Option<String>
}

fn main() {
  let args = Args::parse();
  let rom_path = args.path;
  println!(
    "[ {} v.{} (built on {}) ]",
    NAME.unwrap_or("<name?>"),
    VERSION.unwrap_or("<version?>"),
    BUILD_TIME
  );
  let mut gb = GameboyBuilder::new()
    .init(true)
    .skip_bootrom(args.skip_bootrom)
    .build();
  if args.nogui {
    gb.load_rom_file(
      rom_path.expect("No ROM path specified").as_str()
    ).expect("Failed to load the ROM file");
  } else {
    gb.pause();
    if let Some(rom_path) = rom_path {
      gb.load_rom_file(
        rom_path.as_str()
      ).expect("Failed to load the ROM file");
      gb.resume();
    }
  }
  
  if args.nogui {
    gb.run().unwrap();
  } else {
    #[cfg(not(feature = "gui"))]
    panic!("No GUI support, use the --nogui (-n) flag or build {} with 'gui' feature", NAME.unwrap_or(""));
    #[cfg(feature = "gui")] {
      gui::GuiState::new(gb).init();
    }
  }
}
 
