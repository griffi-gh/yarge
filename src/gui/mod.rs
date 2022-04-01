use framework::{
  egui, InitProperties,
  Gui, Dimensions as Dim
};
use egui::{Context, RichText, TextStyle, Color32};
use std::{
  fs,
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
const GB_PALETTE: [[u8; 4]; 4] = [
  [0xe0, 0xf8, 0xd0, 0xff],
  [0x88, 0xc0, 0x70, 0xff],
  [0x34, 0x68, 0x56, 0xff],
  [0x08, 0x18, 0x20, 0xff],
];

pub struct GuiState {
  gb: Gameboy,
  gb_result: Result<(), Box<dyn Error>>,
  show_mem_view: bool
}
impl GuiState {
  pub fn new(gb: Gameboy) -> Self {
    Self {
      gb,
      gb_result: Ok(()),
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
  fn prepare(&mut self) {
    if self.gb_result.is_ok() {
      self.gb_result = self.gb.run_for_frame();
    }
  }
  fn render(&mut self, frame: &mut [u8]) {
    let data = self.gb.get_display_data();
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
      pixel.copy_from_slice(&GB_PALETTE[(data[i] & 3) as usize]);
    }
  }
  fn gui(&mut self, ui: &Context, _dim: Dim<f32>) -> bool {
    let mut exit = false;

    let mut reset = |gb: &mut Gameboy| {
      gb.reset();
      gb.pause();
      //self.gb_result = Ok(());
    };

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
            reset(&mut self.gb);
          }
        });
        ui.add_space(2.);
      });
    };
    fn load_dialog(gb: &mut Gameboy) {
      let files = FileDialog::new()
        .add_filter("Nintendo Gameboy ROM file", &["gb", "gbc"])
        .set_directory("/")
        .pick_file();
      if let Some(files) = files {
        let data = fs::read(files);
        if let Ok(data) = data {
          let data_ref = &data[..];
          gb.load_rom(data_ref);
        }
      }
    }

    // HANDLE ERROR
    if self.gb_result.is_err() {
      let str = self.gb_result.as_ref().unwrap_err().to_string();
      error_window(format!(
        "{} error", 
        NAME.unwrap_or("emulator")).as_str(), 
        Color32::YELLOW, 
        str.as_str(), 
        "err_panel"
      );
    }

    // MAIN WINDOW
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {  
      egui::menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
          if ui.button("Load ROM...").clicked() {
            ui.close_menu();
            reset(&mut self.gb);
            load_dialog(&mut self.gb);
          }
          if ui.button("Load ROM (No reset)...").clicked() {
            ui.close_menu();
            load_dialog(&mut self.gb);
          }
          if ui.button("Exit").clicked() {
            exit = true;
          }
        });
        ui.menu_button("Emulation", |ui| {
          if ui.button("Reset").clicked() {
            ui.close_menu();
            reset(&mut self.gb);
          }
          ui.add_enabled_ui(!self.gb.cpu.mmu.bios_disabled, |ui| { 
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
      });    
      // Control
      ui.horizontal_wrapped(|ui| {
        //ui.add_enabled_ui(!crashed, |ui| {
        ui.checkbox(
          &mut self.gb.running, 
          "Running"
        ).on_disabled_hover_text("Crashed");
        /*
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
        } */
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
          let details = format!(
            "Bin: {:#010b}_{:08b}\nDec: {}",
            ((value & 0xFF00) >> 8) as u8,
            (value & 0xFF) as u8,
            value
          );
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
            ).on_hover_text(
              details
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
              .on_hover_text(format!("{}\nPause emulation to change", details));
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
        let reg = &mut self.gb.cpu.reg;
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "af", reg.af(), !self.gb.running, 0x10) {
            let v = if v <= 0xF { v << 4 } else { v };
            reg.set_af(v);
          }
          ui.separator();
          if let Some(v) = register_view(ui, "bc", reg.bc(), !self.gb.running, 1) {
            reg.set_bc(v);
          }
        });
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "de", reg.de(), !self.gb.running, 1) {
            reg.set_de(v);
          }
          ui.separator();
          if let Some(v) = register_view(ui, "hl", reg.hl(), !self.gb.running, 1) {
            reg.set_hl(v);
          }
        });
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "sp", reg.sp, !self.gb.running, 1) {
            reg.set_sp(v);
          }
          ui.separator();
          if let Some(v) = register_view(ui, "pc", reg.pc, !self.gb.running, 1) {
            reg.set_pc(v);
          }
        });
      });
      ui.separator();
      {
        ui.label(format!("{} v.{} ({} build)",
          NAME.unwrap_or("<name?>"),
          VERSION.unwrap_or("<version?>"),
          {
            #[cfg(not(debug_assertions))] { "release" }
            #[cfg(debug_assertions)]      { "debug" }
          }
        ));
      }
    });

    if self.show_mem_view {
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
          let mem = {
            let mem_needed = row_amount * 16;
            let mut mem = vec!();
            mem.reserve(0xff);
            for addr in 0..mem_needed {
              mem.push(self.gb.cpu.mmu.rb(addr as u16 + offset))
            }
            mem
          };
          let pc = self.gb.cpu.reg.pc;
          for row in 0..row_amount {
            let row_start = row << 4;
            ui.horizontal(|ui| {
              ui.monospace(format!("{:04X}", row_start + offset as usize));
              ui.separator();
              for col in 0..16_u16 {
                let addr_rel = col | row_start as u16;
                let addr = addr_rel + offset;
                let val = mem[addr_rel as usize];
                ui.label(
                  RichText::new(
                    format!("{:02X}", val)
                  ).monospace().color(
                    if pc == addr { Color32::LIGHT_RED } else { Color32::WHITE }
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

    return exit;
  }
}
