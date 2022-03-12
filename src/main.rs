pub mod gb;
use gb::Gameboy;
use std::{thread, sync::{Arc, Mutex}};

fn start_emulation_thread(gb: &Arc<Mutex<Gameboy>>) -> thread::JoinHandle<()> {   
    let gb = Arc::clone(&gb);
    thread::spawn(move || {
        loop {
            let mut gb = gb.lock().unwrap();
            gb.step();
            drop(gb);
        }
    })
}
fn main() {
    let gb = Arc::new(Mutex::new(Gameboy::new()));
    Gameboy::run_thread(&gb);
    let thread = start_emulation_thread(&gb);
    thread.join().unwrap();
}
