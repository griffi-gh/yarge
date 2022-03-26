use framework::{
  egui, InitProperties,
  Gui, Dimensions as Dim
};
use egui::{Context, RichText, TextStyle, Color32};
use std::{
  fs,
  sync::{Mutex, Arc},
  error::Error,
  hash::Hasher as _,
};
use super::{gb::Gameboy, NAME, VERSION}; 
mod error_words;
use error_words::WORDS as ERROR_WORDS;
use ahash::AHasher;
use rfd::FileDialog;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 4;

pub struct GuiState {
  gb: Arc<Mutex<Gameboy>>,
  show_mem_view: bool
}
impl GuiState {
  pub fn new(gb: Arc<Mutex<Gameboy>>) -> Self {
    Self {
      gb,
      show_mem_view: false,
    }
  }
  ///Warning: consumes self!
  pub fn init(self) {
    framework::init(Box::new(self), InitProperties {
      title: NAME.unwrap_or("open source gameboy emulator"),
      pixels_resoltion: (WIDTH, HEIGHT),
      min_size: (WIDTH, HEIGHT),
      size: (WIDTH * SCALE, HEIGHT * SCALE),
    });
  }
}

impl Gui for GuiState {
  fn render(&mut self, frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      let x = (i % WIDTH as usize) as u32;
      let y = (i / WIDTH as usize) as u32;
      let c: u8 = ((x + y) & 1) as u8 * 0xff;
      let rgba = [c, c , c, 0xff];
      pixel.copy_from_slice(&rgba);
    }
  }
  fn gui(&mut self, ui: &Context, _dim: Dim<f32>) -> bool {
    let mut exit = false;

    let reset_if_crashed = || {
      {
        let mut gb = match self.gb.lock() {
          Ok(gb) => { gb },
          Err(gb) => { gb.into_inner() }
        };
        gb.pause();
        gb._reset();
      }
      Gameboy::run_thread(&self.gb);
    };

    //ERROR WINDOW
    //MAYBE use error type instead of message to generate error code?
    let mut error_window_reset_after_drop = false;
    let mut error_window = |title: &str, color: Color32, details: &str, id: &str| {
      egui::TopBottomPanel::new(
        egui::panel::TopBottomSide::Top, 
        format!("error_panel_{}", id).as_str()
      ).resizable(false).show(ui, |ui| {
        let error_code = {
          let mut error_code = String::new();
          let mut hasher = AHasher::new_with_keys(0, 0);
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
          egui::warn_if_debug_build(ui);
          ui.label(details);
          ui.label("Check console output for more details");
        });
        ui.vertical_centered_justified(|ui| {
          if ui.button("Exit").clicked() {
            exit = true;
          }
          if ui.button("Reset").clicked() {
            error_window_reset_after_drop = true;
          }
        });
        ui.add_space(2.);
      });
    };
    //FILE LOAD DIALOG
    let load_dialog = || {
      let files = FileDialog::new()
        .add_filter("Nintendo Gameboy ROM file", &["gb", "gbc"])
        .set_directory("/")
        .pick_file();
      if let Some(files) = files {
        let data = fs::read(files);
        if let Ok(data) = data {
          let data_ref = &data[..];
          self.gb.lock().unwrap().load_rom(data_ref);
        }
      }
    };

    let mut crashed = false;

    //HANDLE PANIC/POISON
    let mut gb = match self.gb.lock() {
      Ok(gb) => { gb },
      Err(err) => {
        let mut err_info = format!("{}", err);
        if let Some(source) = err.source() {
          err_info += format!("\nCaused by: {}", source).as_str();
        }
        error_window(
          format!(
            "{} thread panicked",
            NAME.unwrap_or("emulator")
          ).as_str(),
          Color32::RED,
          err_info.as_str(),
          "panic_panel"
        );
        crashed = true;
        err.into_inner()
      }
    };

    // TODO - HANDLE ERROR
    if let Some(info) = &gb.thread_info {
      if info.error.is_some() {
        let str = info.error.as_ref().unwrap().as_str();
        drop(info);
        error_window(format!(
          "{} error", 
          NAME.unwrap_or("emulator")).as_str(), 
          Color32::YELLOW, 
          str, 
          "err_panel"
        );
        crashed = true;
      }
    }
    let gb_running = gb.running && !crashed;
    let mut gb_running_raw = gb.running;
    let gb_reg_af = gb.cpu.reg.af();
    let gb_reg_bc = gb.cpu.reg.bc();
    let gb_reg_de = gb.cpu.reg.de();
    let gb_reg_hl = gb.cpu.reg.hl();
    let gb_reg_sp = gb.cpu.reg.sp;
    let gb_reg_pc = gb.cpu.reg.pc;
    let gb_bios_disabled = gb.cpu.mmu.bios_disabled;
    let gb_thread_info = gb.thread_info.clone();
    if gb.thread_info.is_some() {
      let t = gb.thread_info.as_mut().unwrap();
      t.instrs = 0;  
      t.time = std::time::Instant::now();
    }
    drop(gb);

    if error_window_reset_after_drop {
      reset_if_crashed();
    }

    let crashed = crashed;
    let allow_edit = !(gb_running_raw || crashed);

    // MAIN WINDOW
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {  
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          ui.add_enabled_ui(!crashed, |ui| { 
            if ui.button("Load ROM...").clicked() {
              ui.close_menu();
              { 
                let mut gb = self.gb.lock().unwrap();
                gb._reset();
                gb.pause();
              }
              load_dialog();
            }
            if ui.button("Load ROM (No reset)...").clicked() {
              ui.close_menu();
              load_dialog();
            }
          });
          if ui.button("Exit").clicked() {
            exit = true;
          }
        });
        ui.menu_button("Emulation", |ui| {
          if ui.button("Reset").clicked() {
            ui.close_menu();
            if !crashed {
              self.gb.lock().unwrap()._reset();
            } else {
              reset_if_crashed();
            }
          }
          ui.add_enabled_ui(!(gb_bios_disabled || crashed), |ui| { 
            if ui.button("Skip bootrom").clicked() {
              ui.close_menu();
              self.gb.lock().unwrap().skip_bootrom();
            }
          });
        });
        ui.menu_button("Tools", |ui| {
          ui.add_enabled_ui(!(self.show_mem_view || crashed), |ui| {
            if ui.button("Memory view").clicked() {
              ui.close_menu();
              self.show_mem_view = true;
            }
          });
        });
      });    
      // Control
      ui.horizontal_wrapped(|ui| {
        ui.add_enabled_ui(!crashed, |ui| {
          let mut temp = false;
          if ui.checkbox(
            if crashed { &mut temp } else { &mut gb_running_raw }, 
            "Running"
          ).on_disabled_hover_text("Crashed, unable to resume").changed() {
            self.gb.lock().unwrap().running = gb_running_raw;
          }
        });
        if gb_thread_info.is_some() {
          let info = gb_thread_info.unwrap();
          let elapsed = info.time.elapsed().as_secs_f64();
          if gb_running {
            ui.label(format!(
              "~{} IPS", ((info.instrs as f64) / elapsed).round() as u64
            ));
            /*info.time = std::time::Instant::now();
            info.instrs = 0;*/
          } else {
            ui.label(if crashed { "Crashed" } else { "Paused"});
          }
        }
      });

      // Registers
      fn register_view(ui: &mut egui::Ui, name: &str, value: u16, allow_edit: bool, mul: u16) -> Option<u16> {
        let mut ret = None;
        ui.horizontal(|ui| {
          ui.add_enabled_ui(allow_edit, |ui| {
            if ui.button(
              RichText::new("-").monospace()
            ).on_hover_text(format!("-{:#X}", mul)).clicked() {
              ret = Some(value.wrapping_sub(mul));
            }
          });
          ui.monospace(name.to_uppercase());
          if allow_edit {
            let text_style = TextStyle::Monospace;
            let w = egui::WidgetText::from("0000").into_galley(
                ui, 
                Some(false), 
                f32::MAX, 
                text_style.clone()
              ).galley().size().x;
            let mut value_str = format!("{:X}", value).to_string();
            let was_zero = value == 0;
            let res = ui.add(
              egui::TextEdit::singleline(&mut value_str)
                .font(text_style)
                .cursor_at_end(true)
                .desired_width(w)
                .id_source("regview_".to_string() + name)
                .hint_text("0")
                .margin(egui::Vec2::from((0.,0.)))
            );
            if res.changed() {
              if was_zero {
                value_str = value_str.replace("0", "");
              }
              let x = u16::from_str_radix(
                ("0".to_string() + value_str.trim()).as_str(), 
                16
              );
              if x.is_ok() {
                ret = Some(x.unwrap());
              }
            }
          } else {
            ui.monospace(format!("{:04X}", value))
              .on_hover_text("Pause emulation to change");
          }
        });
        ui.add_enabled_ui(allow_edit, |ui| {
          if ui.button(
            RichText::new("+").monospace()
          ).on_hover_text(format!("+{:#X}", mul)).clicked() {
            ret = Some(value.wrapping_add(mul));
          }
        });
        ret
      }
      egui::CollapsingHeader::new(
        "Registers"
      ).default_open(true).show(ui, |ui| {
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "af", gb_reg_af, allow_edit, 0x10) {
            let v = if v <= 0xF { v << 4 } else { v };
            self.gb.lock().unwrap().cpu.reg.set_af(v);
          }
          ui.separator();
          if let Some(v) = register_view(ui, "bc", gb_reg_bc, allow_edit, 1) {
            self.gb.lock().unwrap().cpu.reg.set_bc(v);
          }
        });
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "de", gb_reg_de, allow_edit, 1) {
            self.gb.lock().unwrap().cpu.reg.set_de(v);
          }
          ui.separator();
          if let Some(v) = register_view(ui, "hl", gb_reg_hl, allow_edit, 1) {
            self.gb.lock().unwrap().cpu.reg.set_hl(v);
          }
        });
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "sp", gb_reg_sp, allow_edit, 1) {
            self.gb.lock().unwrap().cpu.reg.set_sp(v);
          }
          ui.separator();
          if let Some(v) = register_view(ui, "pc", gb_reg_pc, allow_edit, 1) {
            self.gb.lock().unwrap().cpu.reg.set_pc(v);
          }
        });
      });
      ui.separator();
      {
        #[cfg(debug_assertions)]
        let build_type = "debug";
        #[cfg(not(debug_assertions))]
        let build_type = "release";
        ui.label(format!("{} v.{} ({} build)",
          NAME.unwrap_or("<name?>"),
          VERSION.unwrap_or("<version?>"),
          build_type
        ));
      }
    });

    if self.show_mem_view && !crashed {
      egui::Window::new("Memory view").open(&mut self.show_mem_view).show(ui, |ui| {
        let height = ui.text_style_height(&egui::TextStyle::Monospace);
        ui.horizontal(|ui| {
          egui::Label::new(RichText::new("0000").monospace()).layout_in_ui(ui);
          ui.separator();
          for i in 0..=0xF_u8 {
            ui.monospace(format!("+{:X}", i));
          }
        });
        ui.separator();
        egui::ScrollArea::vertical().always_show_scroll(true).hscroll(false).vscroll(true).show_rows(ui, height, 0x1000,|ui, row_range| {
          let offset = (row_range.start as u16) << 4;
          let row_amount = row_range.end - row_range.start;
          let (mem,  pc) = {
            let mem_needed = row_amount * 16;
            let mut mem = vec!();
            mem.reserve(0xff);
            let gb = self.gb.lock().unwrap();
            'mem_read: for addr in 0..mem_needed {
              if (addr + offset as usize) > 0xFFFF {
                break 'mem_read;
              }
              mem.push(gb.cpu.mmu.rb((addr as u16) + offset))
            }
            (mem, gb.cpu.reg.pc)
          };
          for row in 0..row_amount {
            let row_start = row << 4;
            ui.horizontal(|ui| {
              ui.monospace(format!("{:04X}", row_start + offset as usize));
              ui.separator();
              for col in 0..16_u16 {
                let addr_rel = col | row_start as u16;
                let addr = addr_rel + offset;
                ui.label(
                  RichText::new(
                    format!("{:02X}", mem[addr_rel as usize])
                  ).monospace().color(
                    if pc == addr { Color32::LIGHT_RED } else { Color32::WHITE }
                  )
                ).on_hover_text(
                  format!("{:#06X}", addr)
                );
              }
            });
          }
        });
      });
    }

    return exit;
  }
}
