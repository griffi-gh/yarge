#![cfg_attr(target_os = "windows", cfg_attr(feature = "production", windows_subsystem = "windows"))]

use yarge_core::{
  Gameboy,
  Key as GbKey,
  consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT}
};
use sdl2::{
  pixels::PixelFormatEnum, 
  event::Event, 
  keyboard::Scancode,
  render::BlendMode,
};
use clap::Parser;

mod audio;
mod menu;
mod anim;
mod text;
use audio::AudioDevice;
use menu::Menu;
use text::TextRenderer;

const FONT_TEXTURE: &[u8] = include_bytes!("../font.rgba");
const FONT_TEXTURE_SIZE: (u32, u32) = (256, 368);

const GB_PALETTE: [u32; 4] = [0x00ffffff, 0x00aaaaaa, 0x00555555, 0x0000000];
const GB_KEYBIND: &[(Scancode, GbKey)] = &[
  (Scancode::Z,       GbKey::A),
  (Scancode::X,       GbKey::B),
  (Scancode::Return,  GbKey::Start),
  (Scancode::Space,   GbKey::Select),
  (Scancode::Up,      GbKey::Up),
  (Scancode::Left,    GbKey::Left),
  (Scancode::Right,   GbKey::Right),
  (Scancode::Down,    GbKey::Down)
];

#[derive(Parser, Debug)]
#[command()]
struct Args {
  rom_path: String,
  #[arg(long)] skip_bootrom: bool,
  #[arg(long, default_value_t = 2)] scale: u32,
  #[arg(long)] fullscreen: bool,
  #[arg(long)] fullscreen_native: bool,
  #[arg(long)] no_vsync: bool,
  #[arg(long, default_value_t = 1)] speed: usize,
}

fn main() {
  //Parse arguments
  let args = Args::parse();

  //Create a Gameboy struct
  let mut gb = Gameboy::new();

  //Load the ROM file
  let rom = std::fs::read(args.rom_path).expect("Failed to load the ROM file");
  gb.load_rom(&rom).expect("Invalid ROM file");

  //Skip bootrom
  if args.skip_bootrom {
    gb.skip_bootrom();
  }

  //Initialize SDL2 Context, VideoSubsystem, Window, EventPump and Canvas
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = {
    let mut builder = video_subsystem.window(
      "YargeSDL", 
      GB_WIDTH as u32 * args.scale,
      GB_HEIGHT as u32 * args.scale
    );
    builder.position_centered();
    if args.fullscreen {
      match args.fullscreen_native {
        true  => builder.fullscreen(),
        false => builder.fullscreen_desktop(),
      };
    }
    //builder.resizable();
    builder.build().unwrap()
  };
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut canvas = {
    let mut builder = window.into_canvas();
    if !args.no_vsync {
      builder = builder.present_vsync();
    }
    builder.build().unwrap()
  };
  canvas.set_blend_mode(BlendMode::Blend);
  
  //Get a texture creator
  let texture_creator = canvas.texture_creator();

  //Create a texture for the screen
  let mut gb_texture = texture_creator.create_texture_streaming(
    PixelFormatEnum::RGB24,
    GB_WIDTH as u32, 
    GB_HEIGHT as u32
  ).unwrap();

  //Create the font texture
  let mut font_texture = texture_creator.create_texture_static(
    PixelFormatEnum::ARGB8888,
    FONT_TEXTURE_SIZE.0,
    FONT_TEXTURE_SIZE.1,
  ).unwrap();
  font_texture.set_blend_mode(BlendMode::Blend);
  font_texture.update(None, FONT_TEXTURE, 4 * FONT_TEXTURE_SIZE.0 as usize).unwrap();

  //Create text renderer
  let text_renderer = TextRenderer::new(&font_texture, (8, 16), 32);

  //Create the audio device and assign it
  let audio_device = AudioDevice::new(&sdl_context).unwrap();
  gb.set_audio_device(audio_device);

  //Create a Menu object that handles the ESC-menu
  let mut menu = Menu::new();

  //Main loop
  'run: loop {
    //Process SDL2 events
    for event in event_pump.poll_iter() {
      menu.process_evt(&event);
      match event {
        Event::Quit {..} => {
          break 'run
        }
        _ => {}
      }
    }
    if menu.is_visible() {
      menu.update(&mut canvas, &gb_texture, &text_renderer);
    } else {
      //Update Gameboy key state
      let kb_state = event_pump.keyboard_state();
      for (scancode, key) in GB_KEYBIND {
        gb.set_key_state(*key, kb_state.is_scancode_pressed(*scancode));
      }
      //Run emulation for one frame
      for _ in 0..args.speed {
        gb.run_for_frame().unwrap();
      }
      //Copy data to texture
      let gb_data = gb.get_display_data();
      gb_texture.with_lock(None, |tex_data: &mut [u8], _pitch: usize| {
        for (index, color) in gb_data.iter().enumerate() {
          let mapped_color = GB_PALETTE[*color as usize];
          tex_data[3 * index] = mapped_color as u8;
          tex_data[(3 * index) + 1] = (mapped_color >> 8) as u8;
          tex_data[(3 * index) + 2] = (mapped_color >> 16) as u8;
        }
      }).unwrap();
      //Copy texture to the entire canvas
      canvas.copy(&gb_texture, None, None).unwrap();
    }
    //Draw canvas
    canvas.present();
  }
}
