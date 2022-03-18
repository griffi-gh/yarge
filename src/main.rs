#![forbid(unsafe_code)]

pub mod gb;
use gb::{Gameboy, GameboyBuilder};
use std::{env,sync::{Arc, Mutex}};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    skip_bootrom: bool,
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
    Gameboy::run_thread(&gb).join().unwrap();
}
