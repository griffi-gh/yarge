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
mod data_dir;
mod config;
use audio::AudioDevice;
use menu::Menu;
use text::TextRenderer;
use config::{Configuration, WindowScale};

const FAT_TEXTURE: &[u8] = include_bytes!("../yoshi.rgb");
const FONT_TEXTURE: &[u8] = include_bytes!("../font.rgba");
const FONT_TEXTURE_SIZE: (u32, u32) = (256, 368);
const FONT_CHAR_SIZE: (u32, u32) = (8, 16);
const FONT_CHARS_PER_LINE: u32 = FONT_TEXTURE_SIZE.0 / FONT_CHAR_SIZE.0;

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
  rom_path: Option<String>,
  #[arg(long)] skip_bootrom: bool,
  #[arg(long, default_value_t = 1)] speed: usize,
}

fn main() {
  //Parse arguments
  let args = Args::parse();

  println!("[INIT/INFO] Initializing configuration system");

  //Read config
  let mut config = Configuration::load_or_default();

  if config.closed_properly {
    //Mark config as dirty
    config.save_dirty().unwrap();
  } else {
    println!("[INIT/WARN] Improper exit detected (configuration file dirty)");
  }
  
  println!("[INIT/INFO] Initializing emulation");

  //Create a Gameboy struct
  let mut gb = Gameboy::new();

  println!("[INIT/INFO] Loading ROM file");

  //Load the ROM file
  if let Some(path) = args.rom_path.as_ref() {
    let rom = std::fs::read(path).expect("Failed to load the ROM file");
    gb.load_rom(&rom).expect("Invalid ROM file");
  }

  //Skip bootrom
  if args.skip_bootrom {
    gb.skip_bootrom();
  }

  println!("[INIT/INFO] Initializing SDL2");

  //Initialize SDL2 Context, VideoSubsystem, Window, EventPump and Canvas
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = {
    let mut builder = video_subsystem.window(
      "YargeSDL", 
      GB_WIDTH as u32 * config.scale.scale_or_default(),
      GB_HEIGHT as u32 * config.scale.scale_or_default()
    );
    builder.position_centered();
    match config.scale {
      WindowScale::Fullscreen => { builder.fullscreen_desktop(); },
      WindowScale::Maximized  => { builder.maximized(); }, 
      _ => ()
    };
    //builder.resizable();
    builder.build().unwrap()
  };
  let mut event_pump = sdl_context.event_pump().unwrap();
  let mut canvas = {
    let mut builder = window.into_canvas();
    // if !args.no_vsync
    builder = builder.present_vsync();
    builder.build().unwrap()
  };
  canvas.set_blend_mode(BlendMode::Blend);
  
  println!("[INIT/INFO] Creating textures");

  //Get a texture creator
  let texture_creator = canvas.texture_creator();

  //Create a texture for the screen
  let mut gb_texture = texture_creator.create_texture_streaming(
    PixelFormatEnum::RGB24,
    GB_WIDTH as u32, 
    GB_HEIGHT as u32
  ).unwrap();
  gb_texture.update(None, FAT_TEXTURE, 3 * GB_WIDTH).unwrap();

  //Create the font texture
  let mut font_texture = texture_creator.create_texture_static(
    PixelFormatEnum::ARGB8888,
    FONT_TEXTURE_SIZE.0,
    FONT_TEXTURE_SIZE.1,
  ).unwrap();
  font_texture.update(
    None, 
    FONT_TEXTURE, 
    4 * FONT_TEXTURE_SIZE.0 as usize
  ).unwrap();
  font_texture.set_blend_mode(BlendMode::Blend);

  //Create text renderer
  let mut text_renderer = TextRenderer::new(
    font_texture, 
    FONT_CHAR_SIZE,
    FONT_CHARS_PER_LINE
  );

  println!("[INIT/INFO] Initializing audio");

  //Create the audio device and assign it
  let audio_device = AudioDevice::new(&sdl_context).unwrap();
  gb.set_audio_device(audio_device);

  println!("[INIT/INFO] Creating menu");

  //Create a Menu object that handles the ESC-menu
  let mut menu = Menu::new();

  //Check close status 
  if !config.closed_properly {
    menu.closed_improperly();
    menu.skip_activation_animation();
  }

  //Activate the menu right away if no rom is loaded
  if args.rom_path.is_none() {
    menu.set_activated_state(true);
    menu.skip_activation_animation();
  }

  println!("[INIT/INFO] Initialization done");

  //Main loop
  'run: loop {
    //Process SDL2 events
    for event in event_pump.poll_iter() {
      menu.process_evt(&event);
      if let Event::Quit {..} = event {
        break 'run
      }
    }
    menu.always_update(&gb);
    if menu.is_visible() {
      let mut exit_signal = false;
      menu.update(
        &mut canvas,
        &mut gb,
        &gb_texture,
        &mut text_renderer,
        &mut config,
        &mut exit_signal
      );
      if exit_signal {
        break 'run;
      }
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
      let palette = config.palette.get_map();
      gb_texture.with_lock(None, |tex_data: &mut [u8], _pitch: usize| {
        for (index, color) in gb_data.iter().enumerate() {
          let mapped_color = palette[*color as usize];
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

  println!("[EXIT/INFO] Starting clean exit procedure...");

  //Save options
  config.save_clean().unwrap();

  println!("[EXIT/INFO] Clean exit done");

  println!("Goodbye")
}
