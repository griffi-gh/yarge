use yarge_gui_framework as framework;
use framework::{
  VirtualKeyCode,
  WinitInputHelper,
  egui, InitProperties,
  Gui, Dimensions as Dim,
  Icon
};
use egui::{Context, RichText, TextStyle, Color32};
use std::{
  fs,
  error::Error,
  hash::Hasher as _,
  time::Instant,
};
pub(crate) use crate::gb;
use crate::{
  gb::consts::{MBC_TYPE_LIST, MBC_TYPE_NAMES},
  gb::{CpuState, Gameboy},
  NAME,
  VERSION,
  GITHUB_REPO,
}; 
use fxhash::FxHasher64;
use rfd::FileDialog;

mod error_words;
mod icons;
mod u16edit;
use u16edit::u16_edit;
use error_words::WORDS as ERROR_WORDS;

#[cfg(feature = "audio")] mod audio;

pub(crate) use gb::consts::{WIDTH as GB_WIDTH, HEIGHT as GB_HEIGHT};
const WIDTH: u32 = GB_WIDTH as u32;
const HEIGHT: u32 = GB_HEIGHT as u32;
const SCALE: u32 = 4;
const GB_PALETTE: [[u8; 4]; 4] = [
  [0xe0, 0xf8, 0xd0, 0xff],
  [0x88, 0xc0, 0x70, 0xff],
  [0x34, 0x68, 0x56, 0xff],
  [0x08, 0x18, 0x20, 0xff],
];

pub struct GuiState {
  gb: Gameboy,
  gb_running: bool,

  gb_result: Result<(), gb::YargeError>,
  show_mem_view: bool,
  load_force_mbc: bool,
  load_force_mbc_type: u8,
  load_no_reset: bool,
  step_amount: usize,
  step_millis: f64,
  last_render: Instant,
  frame_time: f64,
  enable_gui: bool,
  speed: u8,
  corrupt_amount: u16,

  #[cfg(feature = "dbg-breakpoints")]
  mmu_breakpoint_addr: u16,
  #[cfg(feature = "dbg-breakpoints")]
  pc_breakpoint_addr: u16,
}
impl GuiState {
  pub fn new(gb: Gameboy) -> Self {
    Self {
      gb,
      gb_running: false,

      gb_result: Ok(()),
      show_mem_view: false,
      load_force_mbc: false,
      load_force_mbc_type: 0,
      load_no_reset: false,
      step_amount: 1,
      step_millis: 0.,
      last_render: Instant::now(),
      frame_time: 0.,
      enable_gui: true,
      speed: 1,
      corrupt_amount: 100,

      #[cfg(feature = "dbg-breakpoints")]
      mmu_breakpoint_addr: 0,
      #[cfg(feature = "dbg-breakpoints")]
      pc_breakpoint_addr: 0,
    }
  }
  ///Warning: consumes self!
  pub fn init(self) {
    #[cfg(feature = "audio")] audio::init();
    framework::init(self, InitProperties {
      title: NAME.unwrap_or("open source gameboy emulator"),
      pixels_resoltion: (WIDTH, HEIGHT),
      min_size: (WIDTH, HEIGHT),
      size: (WIDTH * SCALE, HEIGHT * SCALE),
      window_icon: Some(Icon::from_rgba(Vec::from(&icons::ICON_WINDOW[..]), 64, 64).unwrap()),
      #[cfg(target_os = "windows")]
      taskbar_icon: Some(Icon::from_rgba(Vec::from(&icons::ICON_TASKBAR[..]), 256, 256).unwrap()),
      #[cfg(not(target_os = "windows"))]
      taskbar_icon: None,
    });
  }
}

impl Gui for GuiState {
  fn prepare(&mut self) {
    let instant = Instant::now();
    if self.gb_result.is_ok() {
      for _ in 0..self.speed {
        self.gb_result = self.gb.run_for_frame();
        if self.gb_result.is_err() {
          break
        }
      }
    }
    let elapsed = instant.elapsed();
    self.step_millis = elapsed.as_secs_f64() * 1000.;
  }
  fn render(&mut self, frame: &mut [u8]) {
    self.frame_time = self.last_render.elapsed().as_secs_f64();
    self.last_render = Instant::now();
    let data = self.gb.get_display_data();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      pixel.copy_from_slice(&GB_PALETTE[(data[i] & 3) as usize]);
    }
  }
  fn handle_input(&mut self, input: &WinitInputHelper) {
    
    if let Some(file) = input.dropped_file() {
      if let Ok(data) = fs::read(file) {
        self.gb.reset();
        if self.gb.load_rom(&data[..]).is_ok() {
          self.gb_running = true;
        }
      }
    }
    //update key state
    //TODO make configurable
    {
      use VirtualKeyCode as KbKey;

      //Hide ui
      if input.key_released(KbKey::G) && input.held_control() {
        self.enable_gui ^= true;
      }

      //Update gb keys

      use gb::Key as GbKey;
      const KEY_MAP: [(KbKey, GbKey); 19] = [
        //Action keys
        (KbKey::Q,       GbKey::A),
        (KbKey::Z,       GbKey::A),
        (KbKey::J,       GbKey::A),
        (KbKey::E,       GbKey::B),
        (KbKey::X,       GbKey::B),
        (KbKey::K,       GbKey::B),
        //Select/start
        (KbKey::LShift,  GbKey::Select),
        (KbKey::RShift,  GbKey::Select),
        (KbKey::Space,   GbKey::Select),
        (KbKey::LControl,GbKey::Start),
        (KbKey::Return,  GbKey::Start),
        //Direction keys
        (KbKey::W,       GbKey::Up),
        (KbKey::A,       GbKey::Left),
        (KbKey::S,       GbKey::Down),
        (KbKey::D,       GbKey::Right),      
        (KbKey::Up,      GbKey::Up),
        (KbKey::Left,    GbKey::Left),
        (KbKey::Down,    GbKey::Down),
        (KbKey::Right,   GbKey::Right),
      ];
      let mut state: u8 = 0;
      for (kb_key, gb_key) in KEY_MAP {
        if input.key_held(kb_key) {
          state |= gb_key as u8;
        }
      }
      self.gb.set_key_state_all(state);
    }
  }
  fn gui(&mut self, ui: &Context, _dim: Dim<f32>) -> bool {
    if !self.enable_gui { return false; }

    let mut exit = false;

    //TODO fix this ugly shit
    let mut reset_error_window = false;
    let mut error_continue = false;
    let mut reset = |gb: &mut Gameboy| {
      gb.reset();
      self.gb_running = false;
      reset_error_window = true;
    };
    
    let mut error_window = |title: &str, color: Color32, details: &str, id: &str, recoverable: bool| {
      egui::TopBottomPanel::new(
        egui::panel::TopBottomSide::Top, 
        format!("error_panel_{}", id).as_str()
      ).resizable(false).show(ui, |ui| {
        let error_code = {
          let mut error_code = String::new();
          let mut hasher = FxHasher64::default();
          hasher.write(details.as_bytes());
          hasher.write(id.as_bytes());
          let hash = hasher.finish();
          let max_index = ERROR_WORDS.len() - 1;
          for (i, w) in ERROR_WORDS.iter().enumerate() {
            let shift: u8 = (i * 8) as u8;
            if i == max_index {
              error_code += "is ";
            }
            error_code += w[(((hash & (0xFF << shift)) >> shift) & 0xFF) as usize];
            if i != max_index {
              error_code += " ";
            }
          }
          error_code
        };
        ui.vertical_centered(|ui| {
          ui.label(RichText::new(title).color(color).size(18.));
          ui.label(error_code);
        });
        ui.collapsing("Details", |ui| {
          ui.label(details);
          ui.label("Check console output for more details");
        });
        let w = ui.available_width() / 3.;
        egui::Grid::new("error_buttons").num_columns(3).max_col_width(w).show(ui, |ui| {
          ui.vertical_centered_justified(|ui| {
            if ui.button("Exit").clicked() {
              exit = true;
            }
          });
          ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
              reset(&mut self.gb);
            }
          });
          ui.vertical_centered_justified(|ui| {
            ui.add_enabled_ui(recoverable, |ui| {
              if ui.button("Continue").on_disabled_hover_text("This error is not recoverable").clicked() {
                error_continue = true;
              }
            });
          });
          ui.end_row();
        });
        ui.add_space(2.);
      });
    };

    fn load_dialog(gb: &mut Gameboy, force_mbc: Option<u8>) -> Result<bool, Box<dyn Error>> {
      let files = FileDialog::new()
        .add_filter("Nintendo Gameboy ROM file", &["gb", "gbc"])
        .set_directory("/")
        .pick_file();
      if let Some(files) = files {
        let data = fs::read(files);
        if let Ok(data) = data {
          let data_ref = &data[..];
          match force_mbc {
            Some(x) => { gb.load_rom_force_mbc(data_ref, x)?; },
            None => { gb.load_rom(data_ref)?; }
          }
          return Ok(true);
        }
      }
      Ok(false)
    }

    // HANDLE ERROR
    if self.gb_result.is_err() {
      let (str, recoverable) = {
        let error = self.gb_result.as_ref().unwrap_err();
        (error.to_string(), error.is_recoverable())
      };
      error_window(format!(
        "{} error", 
        NAME.unwrap_or("emulator")).as_str(), 
        Color32::YELLOW, 
        str.as_str(), 
        "err_panel",
        recoverable
      );
    }

    // MAIN WINDOW
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {  
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          ui.menu_button("Load ROM...", |ui| {
            let clicked = ui.button("Load ROM...").clicked();
            ui.separator();
            ui.checkbox(&mut self.load_force_mbc, "Force MBC type");
            if self.load_force_mbc {
              ui.menu_button(
                format!(
                  "\"{}\"",
                  if self.load_force_mbc {
                    MBC_TYPE_NAMES.get(&self.load_force_mbc_type).unwrap()
                  } else {
                    &"Disabled"
                  }
                ), 
                |ui| {
                  ui.allocate_at_least(egui::Vec2::new(215.,0.), egui::Sense::hover());
                  egui::ScrollArea::new([false, true]).max_height(150.).show(ui, |ui| {
                    for v in MBC_TYPE_LIST {
                      ui.radio_value(
                        &mut self.load_force_mbc_type, 
                        v.0, v.1
                      );
                    }
                  });
                }
              );
            }
            ui.checkbox(&mut self.load_no_reset, "No reset");
            if clicked {
              ui.close_menu();
              if !self.load_no_reset {
                reset(&mut self.gb);
              }
              let opt = match self.load_force_mbc {
                true => Some(self.load_force_mbc_type),
                false => None
              };
              let _ = load_dialog(&mut self.gb, opt)
                .map_err(|err| { println!("Load error: {err}"); });
            }
          });
          if ui.button("Exit").clicked() {
            exit = true;
          }
        });
        ui.menu_button("Emulation", |ui| {
          if ui.button("Reset").clicked() {
            ui.close_menu();
            reset(&mut self.gb);
          }
          ui.add_enabled_ui(!self.gb.get_bios_disabled(), |ui| { 
            if ui.button("Skip bootrom").clicked() {
              ui.close_menu();
              self.gb.skip_bootrom();
            }
          });
        });
        ui.menu_button("Tools", |ui| {
          ui.add_enabled_ui(!self.show_mem_view, |ui| {
            if ui.button("Memory view").clicked() {
              ui.close_menu();
              self.show_mem_view = true;
            }
          });
        });
        ui.menu_button("Fun", |ui| {
          if ui.button("Corrupt some memory").clicked() {
            ui.close_menu();
            for _ in 0..self.corrupt_amount {
              self.gb.write_mem(fastrand::u16(..), fastrand::u8(..))
            }
          }
          if ui.button("Corrupt only WRAM").clicked() {
            ui.close_menu();
            for _ in 0..self.corrupt_amount {
              self.gb.write_mem(fastrand::u16(0xC000..=0xFDFF), fastrand::u8(..))
            }
          }
          if ui.button("Corrupt only ERAM").clicked() {
            ui.close_menu();
            for _ in 0..self.corrupt_amount {
              self.gb.write_mem(fastrand::u16(0xA000..=0xBFFF), fastrand::u8(..))
            }
          }
          if ui.button("Corrupt only MMIO").clicked() {
            ui.close_menu();
            for _ in 0..self.corrupt_amount {
              self.gb.write_mem(fastrand::u16(0xFF00..=0xFF7F), fastrand::u8(..))
            }
          }
          ui.horizontal(|ui| {
            ui.label("Corruption amount:");
            let corrupt_amount = self.corrupt_amount;
            ui.add(
              egui::DragValue::new(&mut self.corrupt_amount)
                .suffix(if corrupt_amount == 1 { " byte" } else { " bytes"})
            );
          });
          if ui.button("Fully fill VRAM with random values").clicked() {
            ui.close_menu();
            for i in 0x8000..=0x9FFF {
              self.gb.write_mem(i, fastrand::u8(..));
            }
          }
        });
      });    

      // RUN CONTROL
      ui.horizontal(|ui| {
        ui.checkbox(&mut self.gb_running, "Running");
        ui.separator();
        ui.add_enabled_ui(!(self.gb_running || self.gb_result.is_err()), |ui| {
          if ui.button(RichText::new("Run for").monospace()).clicked() {
            for _ in 0..self.step_amount {
              self.gb_result = match self.gb.step() {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
              };
              if self.gb_result.is_err() {
                break;
              }
            }
          }
          ui.add_space(-5.);
          let mut amt = self.step_amount;
          ui.add(
            egui::DragValue::new(&mut amt)
              .suffix(if self.step_amount <= 1 { " step" } else { " steps"})
              .max_decimals(0)
              .clamp_range(1..=usize::MAX)
          );
          self.step_amount = amt;

          ui.separator();

          if ui.button("Run for frame").clicked() {
            self.gb_result = self.gb.run_for_frame();
          }
        });
      });
      ui.add(egui::Slider::new(&mut self.speed, 1..=10).text("Speed"));

      //REGISTERS
      egui::CollapsingHeader::new("Registers").default_open(true).show(ui, |ui| {
        egui::Grid::new("register_layout").num_columns(2).show(ui, |ui| {
          if let Some(v) = u16_edit(ui, "af", self.gb.get_reg_af(), !self.gb_running, 0x10) {
            let v = if v <= 0xF { v << 4 } else { v };
            self.gb.set_reg_af(v);
          }
          if let Some(v) = u16_edit(ui, "bc", self.gb.get_reg_bc(), !self.gb_running, 1) {
            self.gb.set_reg_bc(v);
          }
          ui.end_row();

          if let Some(v) = u16_edit(ui, "de", self.gb.get_reg_de(), !self.gb_running, 1) {
            self.gb.set_reg_de(v);
          }
          if let Some(v) = u16_edit(ui, "hl", self.gb.get_reg_hl(), !self.gb_running, 1) {
            self.gb.set_reg_hl(v);
          }
          ui.end_row();

          if let Some(v) = u16_edit(ui, "sp", self.gb.get_reg_sp(), !self.gb_running, 1) {
            self.gb.set_reg_sp(v);
          }
          if let Some(v) = u16_edit(ui, "pc", self.gb.get_reg_pc(), !self.gb_running, 1) {
            self.gb.set_reg_pc(v);
          }
          ui.end_row();
        });
      });

      //CARTRIDGE INFO
      egui::CollapsingHeader::new(
        "Cartridge"
      ).show(ui, |ui| {
        const H_SPACING: f32 = 10.;
        const V_SPACING: f32 = 3.;
        ui.label("MBC Type");
        ui.horizontal(|ui| {
          ui.add_space(H_SPACING);
          ui.label(self.gb.get_mbc_name());
        });
        ui.add_space(V_SPACING);
        ui.label("ROM Header");
        ui.horizontal(|ui| {
          ui.add_space(H_SPACING);
          ui.label(format!(
            "{}", self.gb.get_rom_header()
          ));
        });
      });
      
      egui::CollapsingHeader::new("CPU").default_open(true).show(ui, |ui| {
        ui.horizontal(|ui| {
          let state = self.gb.get_cpu_state();
          ui.label("State:");
          ui.label(RichText::new(match state {
            CpuState::Running => "Running",
            CpuState::Halt => "Halted",
            CpuState::Stop => "Stopped",
          }).color(match state {
            CpuState::Running => Color32::GREEN,
            CpuState::Halt => Color32::YELLOW,
            CpuState::Stop => Color32::LIGHT_RED, 
          }));
        });
      });

      //BREAKPOINTS
      {
        const ENABLED: bool = {
          #[cfg(not(feature = "dbg-breakpoints"))] { false }
          #[cfg(feature = "dbg-breakpoints")]      { true  }
        };
        ui.add_enabled_ui(ENABLED, |ui| {
          egui::CollapsingHeader::new(
            "Breakpoints"
          ).show(ui, |ui| {
            #[cfg(feature = "dbg-breakpoints")] {
              ui.horizontal(|ui| {
                if let Some(v) = u16_edit(ui, "MMU", self.mmu_breakpoint_addr, true, 1) {
                  self.mmu_breakpoint_addr = v;
                }
                if ui.button("R/W").clicked() {
                  self.gb.set_mmu_breakpoint(
                    self.mmu_breakpoint_addr,
                    0b11
                  )
                }
                if ui.button("R").clicked() {
                  self.gb.set_mmu_breakpoint(
                    self.mmu_breakpoint_addr,
                    0b01
                  )
                }
                if ui.button("W").clicked() {
                  self.gb.set_mmu_breakpoint(
                    self.mmu_breakpoint_addr,
                    0b10
                  )
                }
                if ui.button("Disable").clicked() {
                  self.gb.set_mmu_breakpoint(
                    self.mmu_breakpoint_addr,
                    0b00
                  )
                }
              });
              ui.horizontal(|ui| {
                if let Some(v) = u16_edit(ui, "PC ", self.pc_breakpoint_addr, true, 1) {
                  self.pc_breakpoint_addr = v;
                }
                if ui.button("Enable").clicked() {
                  self.gb.set_pc_breakpoint(
                    self.pc_breakpoint_addr,
                    true
                  );
                }
                if ui.button("Disable").clicked() {
                  self.gb.set_pc_breakpoint(
                    self.pc_breakpoint_addr,
                    false
                  );
                }
              });
              egui::CollapsingHeader::new(
                RichText::new("Warning").color(Color32::LIGHT_YELLOW)
              ).show(ui, |ui| {
                ui.label("Breakpoints are experimental");
                ui.label("1) PC breakpoints get triggered AFTER the instruction is executed");
                ui.label("2) PC breakpoints can cause minor timing issues");
                ui.label("3) MMU breakpoints are NOT recoverable");
              });
            }
          });
        });
      }

      egui::CollapsingHeader::new(
        "Application"
      ).show(ui, |ui| {
        ui.label(format!("Frame time: {}ms", self.frame_time));
        ui.label(format!("\t- Estimated real FPS: {}", (1. / self.frame_time).round() as usize));
        ui.label(format!("gb.run_for_frame() time: {}ms", self.step_millis));
        ui.label(format!("\t- Estimated potential FPS (excl. GUI): {}", (1000. / self.step_millis).round() as usize));
        if ui.button("Organize windows").clicked() {
          ui.ctx().memory().stop_text_input();
          ui.ctx().memory().reset_areas();
        }
        egui::CollapsingHeader::new(
          "Memory"
        ).show(ui, |ui| {
          ui.horizontal(|ui| {
            ui.label("`GuiState` size (on stack): ");
            ui.monospace(std::mem::size_of_val(&*self).to_string());
            ui.label(" bytes");
          });
          ui.horizontal(|ui| {
            ui.label("\t- `Gameboy` size (on stack): ");
            ui.monospace(std::mem::size_of_val(&self.gb).to_string());
            ui.label(" bytes");
          });
        });
      });

      //FOOTER
      ui.label("Press CTRL+G to hide this window");
      ui.separator();
      ui.horizontal(|ui| {
        ui.label(format!("{} v.{} (core: v.{}; {} build)",
          NAME.unwrap_or("<name?>"),
          VERSION.unwrap_or("<version?>"),
          gb::consts::VERSION.unwrap_or("<version?>"),
          {
            #[cfg(debug_assertions)]      { "debug" }
            #[cfg(not(debug_assertions))] { "release" }
          }
        ));
        const TEXT: &str = "GitHub";
        let link_width = egui::WidgetText::from(TEXT).into_galley(
          ui, 
          Some(false), 
          f32::INFINITY,
          TextStyle::Body
        ).galley().size().x;
        ui.add_space((ui.available_width() - link_width).max(0.));
        ui.hyperlink_to("GitHub", GITHUB_REPO);
      });
    });

    //MEMORY VIEW WINDOW
    if self.show_mem_view {
      egui::Window::new("Memory view").open(&mut self.show_mem_view).show(ui, |ui| {
        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        ui.horizontal(|ui| {
          egui::Label::new(RichText::new("0000").monospace()).layout_in_ui(ui);
          for i in 0..=0xF_u8 {
            ui.monospace(format!("+{:X}", i));
          }
        });
        egui::ScrollArea::vertical().always_show_scroll(true).hscroll(false).vscroll(true).show_rows(ui, height, 0x1000,|ui, row_range| {
          let offset = (row_range.start as u16) << 4;
          let row_amount = row_range.end - row_range.start;
          let pc = self.gb.get_reg_pc();
          for row in 0..row_amount {
            let row_start = row << 4;
            ui.horizontal(|ui| {
              ui.monospace(format!("{:04X}", row_start + offset as usize));
              for col in 0..16_u16 {
                let addr_rel = col | row_start as u16;
                let addr = addr_rel + offset;
                let val = self.gb.read_mem(addr);
                ui.label(
                  RichText::new(
                    format!("{:02X}", val)
                  ).monospace().color(
                    if pc == addr {
                      Color32::LIGHT_RED
                    } else {
                      const DEFAULT_COLOR: Color32 = Color32::WHITE;
                      #[cfg(feature = "dbg-breakpoints")]
                      if self.gb.get_pc_breakpoint(addr) {
                        Color32::DARK_GREEN
                      } else if self.gb.get_mmu_breakpoint(addr) > 0 {
                        Color32::DARK_BLUE
                      } else {
                        DEFAULT_COLOR
                      }
                      #[cfg(not(feature = "dbg-breakpoints"))] {
                        DEFAULT_COLOR
                      }
                    }
                  )
                ).on_hover_text(
                  format!("Dec: {0}\nBin: {0:#010b}\nAddr: {1:#06X}", val, addr)
                );
              }
            });
          }
        });
      });
    }

    //TODO ...and this
    if reset_error_window || error_continue {
      self.gb_result = Ok(());
    }
    if error_continue {
      self.gb_running = false;
    }
    
    exit
  }
}
