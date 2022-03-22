use framework::{
  egui, InitProperties,
  Gui, Dimensions as Dim
};
use egui::{Context, RichText, TextStyle, Color32};
use std::{
  sync::{Mutex, Arc},
  error::Error
};
use super::{gb::Gameboy, NAME}; //TODO get rid of dependency on gb

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 3;

pub struct GuiState {
  gb: Arc<Mutex<Gameboy>>,
}
impl GuiState {
  pub fn new(gb: Arc<Mutex<Gameboy>>) -> Self {
    Self {
      gb
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
    let mut error_window = |title: &str, color: Color32, details: &str, id: &str| {
      egui::TopBottomPanel::new(
        egui::panel::TopBottomSide::Top, 
        format!("error_panel_{}", id).as_str()
      ).resizable(false).show(ui, |ui| {
        ui.vertical_centered(|ui| {
          ui.label(RichText::new(title).color(color).size(18.));
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
          ui.add_enabled_ui(false, |ui| {
            ui.button("Restart").on_disabled_hover_text("WIP");
          });
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
    //error_window(format!("{} crashed", NAME.unwrap_or("emulator")).as_str(), Color32::YELLOW, "TODO", "err_panel");

    // MAIN WINDOW
    let gb_running = gb.running && !crashed;

    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {      
      // Control
      {
        let mut temp = false;
        ui.checkbox(
          if crashed { &mut temp } else { &mut gb.running }, 
          "Running"
        );
        drop(temp);
        if gb.thread_info.is_some() {
          let info = gb.thread_info.as_mut().unwrap();
          let elapsed = info.time.elapsed().as_secs_f64();
          if gb_running {
            ui.label(format!(
              "~{} IPS", ((info.instrs as f64) / elapsed).round() as u64
            ));
          } else {
            ui.label("Paused");
          }
          
          info.time = std::time::Instant::now();
          info.instrs = 0;
        }
      }
      // Registers
      fn register_view(ui: &mut egui::Ui, name: &str, value: u16, allow_edit: bool) -> Option<u16> {
        let mut ret = None;
        ui.horizontal(|ui| {
          ui.monospace(name.to_uppercase());
          //works but consumes space in ui
          //let w = egui::Label::new(RichText::new("0000").monospace()).layout_in_ui(ui).1.size().x;
          //Works but how do i get the size??
          //let w: f32 = ui.fonts().glyph_width(&egui::FontId::monospace(/*size*/), '0') * 4.;
          //Is there any better way?
          let w = 30.; //FIXME
          if allow_edit {
            let mut value_str = format!("{:X}", value).to_string();
            let res = ui.add(
              egui::TextEdit::singleline(&mut value_str)
                .font(TextStyle::Monospace)
                .cursor_at_end(true)
                .desired_width(w)
                .id_source("regview_".to_string() + name)
                .hint_text("0000")
                .margin(egui::Vec2::from((0.,0.)))
            );
            if res.changed() {
              let x = u16::from_str_radix(
                ("0".to_string() + value_str.as_str()).as_str(), 
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
        ret
      }
      egui::CollapsingHeader::new(
        "Registers"
      ).default_open(true).show(ui, |ui| {
        let reg = &mut gb.cpu.reg;
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "af", reg.af(), !gb_running) {
            reg.set_af(v);
          }
          if let Some(v) = register_view(ui, "bc", reg.bc(), !gb_running) {
            reg.set_bc(v);
          }
        });
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "de", reg.de(), !gb_running) {
            reg.set_de(v);
          }
          if let Some(v) = register_view(ui, "hl", reg.hl(), !gb_running) {
            reg.set_hl(v);
          }
        });
        ui.horizontal(|ui| {
          if let Some(v) = register_view(ui, "sp", reg.sp(), !gb_running) {
            reg.set_sp(v);
          }
          if let Some(v) = register_view(ui, "pc", reg.pc(), !gb_running) {
            reg.set_pc(v);
          }
        });
      });
    });

    return exit;
  }
}
