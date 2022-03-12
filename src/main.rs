pub mod gb;
use gb::Gameboy;
use std::{thread, sync::{Arc, Mutex}};

fn start_emulation_thread(gb: &Arc<Mutex<Gameboy>>) {
    let gb = Arc::clone(&gb);
    thread::spawn(move || {
        loop {
            let mut gb = gb.lock().unwrap();
            gb.step();
            drop(gb);
        }
    });
}
fn main() {
    let mut gb = Arc::new(Mutex::new(Gameboy::new()));
    start_emulation_thread(&gb);
    
    loop {}
}
