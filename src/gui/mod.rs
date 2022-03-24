use framework::{
  egui, InitProperties,
  Gui, Dimensions as Dim
};
use egui::{Context, RichText, TextStyle, Color32};
use std::{
  sync::{Mutex, Arc},
  error::Error,
  hash::Hasher as _
};
use super::{gb::Gameboy, NAME}; 
mod error_words;
use error_words::WORDS as ERROR_WORDS;
use ahash::AHasher;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 3;

pub struct GuiState {
  gb: Arc<Mutex<Gameboy>>,
}
impl GuiState {
  pub fn new(gb: Arc<Mutex<Gameboy>>) -> Self {
    Self {
      gb,
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
  fn gui(&mut self, ui: &Context, _dim: Dim<f32>) -> bool {
    let mut exit = false;

    //ERROR WINDOW
    //MAYBE use error type instead of message to generate error code?
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
        });
        ui.add_space(2.);
      });
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

    let crashed = crashed;
    let allow_edit = !(gb_running_raw || crashed);

    // MAIN WINDOW
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {      
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
      ui.add_enabled_ui(!gb_bios_disabled, |ui| {
        if ui.button("Skip bootrom").clicked() {
          self.gb.lock().unwrap().skip_bootrom();
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
    });

    return exit;
  }
}
