pub mod gb;
use gb::Gameboy;
use std::sync::{Arc, Mutex};

fn main() {
    let gb = Arc::new(Mutex::new(Gameboy::new()));
    let thread = Gameboy::run_thread(&gb);
    thread.join().unwrap();
}
