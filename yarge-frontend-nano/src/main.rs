use yarge_core::{
  Gameboy,
  consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT}
};
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = GB_WIDTH;
const HEIGHT: usize = GB_HEIGHT;

fn main() {
  //create a minifb window
  let mut window = Window::new(
    "Yarge Nano",
    WIDTH,
    HEIGHT,
    WindowOptions::default(),
  ).unwrap();

  //Limit fps
  window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

  let mut gb = Gameboy::new();
  gb.load_rom_file(std::env::args().nth(1).unwrap().as_str()).unwrap();

  while window.is_open() && !window.is_key_down(Key::Escape) {
    gb.run_for_frame().unwrap();
    let buffer = gb.get_display_data().iter().intersperse(separator);
    window
      .update_with_buffer(&buffer[..], WIDTH, HEIGHT)
      .unwrap();
  }
}