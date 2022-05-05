pub(crate) use yarge_core as gb;
use gb::Gameboy;
use clap::Parser;
use build_time::build_time_local;

pub(crate) const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const BUILD_TIME: &str = build_time_local!("%Y-%m-%dT%H:%M:%S%.f%:z");
pub(crate) const GITHUB_REPO: &str = "https://github.com/griffi-gh/yarge";

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
  let Args { 
    path: rom_path, 
    nogui, 
    skip_bootrom,
  } = args;

  println!(
    "[ {} v.{} ({}; core v. {}; built on {}) ]\n{}/",
    NAME.unwrap_or("<name?>"),
    VERSION.unwrap_or("<version?>"),
    {
      #[cfg(debug_assertions)] { "debug" }
      #[cfg(not(debug_assertions))] { "debug" }
    },
    gb::VERSION.unwrap_or("<version?>"),
    BUILD_TIME,
    GITHUB_REPO
  );

  #[cfg(not(feature = "gui"))]
  if !nogui {
    panic!("No GUI support, use the --nogui (-n) flag or build {} with 'gui' feature", NAME.unwrap_or(""));
  }
  
  let mut gb = Gameboy::new();
  gb.init();
  if skip_bootrom {
    gb.skip_bootrom();
  }

  if nogui {
    gb.load_rom_file(
      rom_path.expect("No ROM path specified").as_str()
    ).expect("Failed to load the ROM file");
    gb.run().unwrap();
  } else {
    gb.pause();
    if let Some(rom_path) = rom_path {
      gb.load_rom_file(
        rom_path.as_str()
      ).expect("Failed to load the ROM file");
      gb.resume();
    }
    #[cfg(feature = "gui")] {
      gui::GuiState::new(gb).init();
    }
  }
}
