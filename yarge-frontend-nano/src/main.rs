use yarge_core::{
  Gameboy,
  Key as GbKey,
  consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT}
};
use minifb::{Key, Window, WindowOptions};

const GB_PALETTE: [u32; 4] = [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x0000000];
const WIDTH: usize = GB_WIDTH;
const HEIGHT: usize = GB_HEIGHT;
const FB_SIZE: usize = WIDTH * HEIGHT;

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
  
  //Create a Gameboy struct and load the rom
  let mut gb = Gameboy::new();
  gb.load_rom_file(std::env::args().nth(1).unwrap().as_str()).unwrap();

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
      ((window.is_key_down(Key::Z) as u8) * (GbKey::A as u8)) |
      ((window.is_key_down(Key::X) as u8) * (GbKey::B as u8)) |
      ((window.is_key_down(Key::RightShift) as u8) * (GbKey::Select as u8)) |
      ((window.is_key_down(Key::Enter) as u8) * (GbKey::Start as u8))
    );

    //Run emulation
    gb.run_for_frame().unwrap();

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