#![cfg_attr(target_os = "windows", cfg_attr(feature = "production", windows_subsystem = "windows"))]

use yarge_core::{
  Gameboy,
  Key as GbKey,
  consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT}
};
use sdl2::{
  pixels::{PixelFormatEnum, Color},
  event::Event,
  keyboard::Scancode,
  render::BlendMode,
};
use clap::Parser;
use std::path::PathBuf;

mod audio;
mod menu;
mod anim;
mod text;
mod data_dir;
mod config;
mod saves;

use audio::AudioDevice;
use menu::Menu;
use text::TextRenderer;
use config::{Configuration, WindowScale};
use saves::SaveManager;

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
}

fn main() {
  //Set dpi aware flag on windows
  #[cfg(all(windows, feature = "hidpi"))] {
    use windows::Win32::UI::HiDpi::{SetProcessDpiAwareness, PROCESS_PER_MONITOR_DPI_AWARE};
    if unsafe { SetProcessDpiAwareness(PROCESS_PER_MONITOR_DPI_AWARE) }.is_err() {
      println!("[ERR] Failed to set DPI awareness");
    }
  }

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
    config.last_rom = Some(path.into());
  }

  //Try to load the save file
  SaveManager::load_idk(&mut gb, config.save_slot);
  SaveManager::save(&gb, config.save_slot).unwrap(); // Call save to create the save file

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
    builder.allow_highdpi();
    match config.scale {
      WindowScale::Fullscreen => { builder.fullscreen_desktop(); },
      WindowScale::Maximized  => { builder.maximized(); },
      _ => ()
    };
    //builder.resizable();
    builder.build().unwrap()
  };
  let mut event_pump = sdl_context.event_pump().unwrap();

  let using_vsync = true; //refresh_rate % 60 == 0;
  let mut canvas = {
    println!("[INIT/INFO] using vsync? {}", if using_vsync { "YES" } else { "NO" });
    let mut builder = window.into_canvas();
    if using_vsync {
      builder = builder.present_vsync();
    }
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
  let mut menu = Menu::new(&config);

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

  #[cfg(feature = "hidpi")]
  let mut dpi_prev = 1.;
  let mut hz_prev = 60;
  
  //Main loop
  'run: loop {
    //Figure out dpi stuff
    let display_dpi_scale = {
      #[cfg(feature = "hidpi")] {
        let mut display_dpi_scale = if config.dpi_scaling {
          video_subsystem.display_dpi(canvas.window().display_index().unwrap()).unwrap_or_else(|_| {
            println!("[WARN/DPI] failed to get display DPI, assuming 96dpi");
            (96., -1., -1.)
          }).0 / 96.
        } else {
          1.
        };
        if !config.dpi_scaling_frac {
          display_dpi_scale = display_dpi_scale.ceil();
        }
        if dpi_prev != display_dpi_scale {
          println!("[INFO/DPI] dpi scale changed from {} to {}", dpi_prev, display_dpi_scale);
          dpi_prev = display_dpi_scale;
          if matches!(config.scale, WindowScale::Fullscreen | WindowScale::Maximized) {
            println!("[WARN/DPI] Not applying dpi scaling to window size as it's either fullscreen or maximized");
          } else {
            let s = (
              GB_WIDTH as u32 * config.scale.scale_or_default(),
              GB_HEIGHT as u32 * config.scale.scale_or_default()
            );
            canvas.window_mut().set_size(
              (display_dpi_scale * s.0 as f32) as u32, 
              (display_dpi_scale * s.1 as f32) as u32
            ).unwrap();
          }
        }
        text_renderer.set_render_dpi_scale(display_dpi_scale);
        display_dpi_scale
      }
      #[cfg(not(feature = "hidpi"))] { 1. }
    };

    //Process SDL2 events
    for event in event_pump.poll_iter() {
      menu.process_evt(&event);
      match event {
        Event::DropFile { filename, .. } => {
          SaveManager::save(&gb, config.save_slot).unwrap();
          let path: PathBuf = filename.into();
          menu.load_file(path.clone(), &mut gb, &config);
          config.last_rom = Some(path);
          config.save_dirty().unwrap();
          menu.set_activated_state(false);
        },
        Event::Quit {..} => break 'run,
        _ => ()
      } 
    }
    menu.always_update(&gb);
    if menu.is_visible() {
      let mut exit_signal = false;
      menu.update(
        &mut canvas,
        display_dpi_scale,
        &mut gb,
        &mut gb_texture,
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
      for _ in 0..config.speed {
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

      //Allow skipping bootrom
      if !gb.get_bios_disabled() {
        text_renderer.set_color(Color::BLACK);
        text_renderer.render(&mut canvas, (0, 0), 1., "Press space to skip\n(Hold Alt to tick)");
        if kb_state.is_scancode_pressed(Scancode::Space) {
          if kb_state.is_scancode_pressed(Scancode::LAlt) | kb_state.is_scancode_pressed(Scancode::RAlt) {
            println!("[INFO] Skipping bootrom [TICKING!!!]");
            while gb.get_reg_pc() < 0x100 { gb.step().unwrap(); }
          } else {
            println!("[INFO] Skipping bootrom");
            gb.skip_bootrom();
          }
        }
      }
    }

    //Draw canvas
    canvas.present();

    //On high-refresh-rate displays, that are multiple of 60 frames, present the same frame multiple times
    //It's ok to run the menu at 120+hz tho, so only do this while emulation is running
    let refresh_rate = canvas.window().display_mode().map(|x| x.refresh_rate).unwrap_or_else(|_| {
      println!("[WARN/UHH] window display mode lookup failed, falling back to monitor 0");
      video_subsystem.display_mode(0, 0).map(|x| x.refresh_rate).unwrap_or_else(|_| {
        println!("[WARN/FUCK] monitor 0 lookup failed, falling back to last successful lookup");
        hz_prev
      })
    });
    hz_prev = refresh_rate;
    if !menu.is_visible() && using_vsync && (refresh_rate % 60) == 0 && refresh_rate > 60 {
      let skip = (refresh_rate / 60) - 1;
      for _ in 0..skip { canvas.present(); }
    }
    //TODO framelimit if refresh rate > 60 but not multiple 60 (e.g. 75/90);
  }

  println!("[EXIT/INFO] Starting clean exit procedure...");

  //Save eram
  SaveManager::save(&gb, config.save_slot).unwrap();

  //Save options
  config.save_clean().unwrap();

  println!("[EXIT/INFO] Clean exit done");

  println!("Goodbye")
}
