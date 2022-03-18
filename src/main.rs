#![forbid(unsafe_code)]

pub mod gb;
use gb::{Gameboy, GameboyBuilder};
use std::{env,sync::{Arc, Mutex}};

fn main() {
    let rom_path = &env::args().nth(1).expect("No ROM path given")[..];
    let gb = GameboyBuilder::new()
        .init()
        .skip_bootrom()
        .load_rom_file(rom_path).expect("Failed to load the ROM file")
        .build();
    let gb = Arc::new(Mutex::new(gb));
    Gameboy::run_thread(&gb).join().unwrap();
}
