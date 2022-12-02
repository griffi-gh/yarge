use yarge_core::{
  Gameboy,
  Key as GbKey,
  consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT}
};
use minifb::{Key, Window, WindowOptions, Scale};
use std::time::Duration;

const GB_PALETTE: [u32; 4] = [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x0000000];
const WIDTH: usize = GB_WIDTH;
const HEIGHT: usize = GB_HEIGHT;
const FB_SIZE: usize = WIDTH * HEIGHT;

fn main() {
  //Create a minifb window
  let mut window = Window::new(
    "Yarge Nano",
    WIDTH,
    HEIGHT,
    WindowOptions { 
      scale: Scale::X2,
      ..Default::default()
    }
  ).expect("Failed to create the window");

  //Limit refresh rate
  window.limit_update_rate(Some(Duration::from_micros(16600)));

  //Create a Gameboy struct
  let mut gb = Gameboy::new();

  //Load ROM file
  gb.load_rom_file(
    std::env::args().nth(1).expect("No ROM path specified").as_str()
  ).expect("Failed to load the ROM file");

  //Create a frame buffer
  let mut framebuffer = Box::new([0u32; FB_SIZE]);

  while window.is_open() && !window.is_key_down(Key::Escape) {
    //Update key state
    //TODO figure out how to do this without multiplication!
    gb.set_key_state_all(
      ((window.is_key_down(Key::Right) as u8) * (GbKey::Right as u8)) |
      ((window.is_key_down(Key::Left) as u8) * (GbKey::Left as u8)) |
      ((window.is_key_down(Key::Up) as u8) * (GbKey::Up as u8)) |
      ((window.is_key_down(Key::Down) as u8) * (GbKey::Down as u8)) |
      (((window.is_key_down(Key::Z) || window.is_key_down(Key::NumPad0)) as u8) * (GbKey::A as u8)) |
      (((window.is_key_down(Key::X) || window.is_key_down(Key::NumPad1)) as u8) * (GbKey::B as u8)) |
      ((window.is_key_down(Key::RightShift) as u8) * (GbKey::Select as u8)) |
      ((window.is_key_down(Key::Enter) as u8) * (GbKey::Start as u8))
    );

    //Run emulation
    let run_frames = 1 + window.is_key_down(Key::LeftShift) as u8;
    for _ in 0..run_frames {
      gb.run_for_frame().unwrap();
    }

    //Update framebuffer
    let data = gb.get_display_data();
    for (index, pixel) in data.iter().enumerate() {
      framebuffer[index] = GB_PALETTE[*pixel as usize];
    }

    //Update window
    window
      .update_with_buffer(&framebuffer[..], WIDTH, HEIGHT)
      .unwrap();
  }
}