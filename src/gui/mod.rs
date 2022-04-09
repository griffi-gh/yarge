use yarge_gui_framework as framework;
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
use crate::{
  gb::consts::{MBC_TYPE_LIST, MBC_TYPE_NAMES},
  gb::Gameboy,
  NAME,
  VERSION,
  GITHUB_REPO,
}; 
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
  show_mem_view: bool,
  load_force_mbc: bool,
  load_force_mbc_type: u8,
  load_no_reset: bool,

  #[cfg(feature = "breakpoints")]
  mmu_breakpoint_addr: u16,
  #[cfg(feature = "breakpoints")]
  pc_breakpoint_addr: u16,
}
impl GuiState {
  pub fn new(gb: Gameboy) -> Self {
    Self {
      gb,
      gb_result: Ok(()),
      show_mem_view: false,
      load_force_mbc: false,
      load_force_mbc_type: 0,
      load_no_reset: false,

      #[cfg(feature = "breakpoints")]
      mmu_breakpoint_addr: 0,
      #[cfg(feature = "breakpoints")]
      pc_breakpoint_addr: 0,
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

fn u16_edit(ui: &mut egui::Ui, name: &str, value: u16, allow_edit: bool, mul: u16) -> Option<u16> {
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
          f32::INFINITY, 
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
    ui.add_enabled_ui(allow_edit, |ui| {
      if ui.button(
        RichText::new("+").monospace()
      ).on_hover_text(format!("+{:#X}", mul)).clicked() {
        ret = Some(value.wrapping_add(mul));
      }
    });
  });
  ret
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

    //TODO fix this ugly shit
    let mut reset_error_window = false;
    let mut reset = |gb: &mut Gameboy| {
      gb.reset();
      gb.pause();
      reset_error_window = true;
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
              //TODO
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
      });    

      // Control
      ui.checkbox(&mut self.gb.running, "Running");

      //REGISTERS
      egui::CollapsingHeader::new("Registers").default_open(true).show(ui, |ui| {
        egui::Grid::new("register_layout").num_columns(2).show(ui, |ui| {
          if let Some(v) = u16_edit(ui, "af", self.gb.get_reg_af(), !self.gb.running, 0x10) {
            let v = if v <= 0xF { v << 4 } else { v };
            self.gb.set_reg_af(v);
          }
          if let Some(v) = u16_edit(ui, "bc", self.gb.get_reg_bc(), !self.gb.running, 1) {
            self.gb.set_reg_bc(v);
          }
          ui.end_row();

          if let Some(v) = u16_edit(ui, "de", self.gb.get_reg_de(), !self.gb.running, 1) {
            self.gb.set_reg_de(v);
          }
          if let Some(v) = u16_edit(ui, "hl", self.gb.get_reg_hl(), !self.gb.running, 1) {
            self.gb.set_reg_hl(v);
          }
          ui.end_row();

          if let Some(v) = u16_edit(ui, "sp", self.gb.get_reg_sp(), !self.gb.running, 1) {
            self.gb.set_reg_sp(v);
          }
          if let Some(v) = u16_edit(ui, "pc", self.gb.get_reg_pc(), !self.gb.running, 1) {
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
          ui.label(format!(
            "{} (with index: {:#04X})",
            self.gb.get_mbc_name(),
            self.gb.get_mbc_type()
          ));
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

      //BREAKPOINTS
      {
        const ENABLED: bool = {
          #[cfg(not(feature = "breakpoints"))] { false }
          #[cfg(feature = "breakpoints")]      { true  }
        };
        ui.add_enabled_ui(ENABLED, |ui| {
          egui::CollapsingHeader::new(
            "Breakpoints"
          ).show(ui, |ui| {
            #[cfg(feature = "breakpoints")] {
              ui.label("WARNING: Breakpoints cause panic lol");
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
            }
          });
        });
      }

      ui.separator();

      //FOOTER
      ui.horizontal(|ui| {
        ui.label(format!("{} v.{} ({} build)",
          NAME.unwrap_or("<name?>"),
          VERSION.unwrap_or("<version?>"),
          {
            #[cfg(not(debug_assertions))] { "release" }
            #[cfg(debug_assertions)]      { "debug" }
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
                let val = self.gb.read_mem(addr as u16);
                ui.label(
                  RichText::new(
                    format!("{:02X}", val)
                  ).monospace().color(
                    if pc == addr {
                      Color32::LIGHT_RED
                    } else if self.gb.get_pc_breakpoint(addr) {
                      Color32::DARK_GREEN
                    } else if self.gb.get_mmu_breakpoint(addr) > 0 {
                      Color32::DARK_BLUE
                    } else {
                      Color32::WHITE
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

    if reset_error_window {
      self.gb_result = Ok(());
    }
    
    return exit;
  }
}
