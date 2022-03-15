pub mod gb;
use gb::Gameboy;
use std::sync::{Arc, Mutex};

fn main() {
    let gb = Arc::new(Mutex::new(Gameboy::new().init()));
    Gameboy::run_thread(&gb).join().unwrap();
}
