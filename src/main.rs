#![forbid(unsafe_code)]

pub mod gb;
use gb::{Gameboy, GameboyBuilder};
use std::sync::{Arc, Mutex};

fn main() {
    let gb = GameboyBuilder::new().init().build().unwrap();
    let gb = Arc::new(Mutex::new(gb));
    Gameboy::run_thread(&gb).join().unwrap();
}
