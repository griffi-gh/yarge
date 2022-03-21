use framework::{
  egui, InitProperties,
  Gui, Dimensions as Dim
};
use egui::{Context, RichText, Color32};
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
          //let _ = ui.button("Restart"); TODO
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

    fn register_view(ui: &mut egui::Ui, name: &str, value: u16) {
      ui.horizontal(|ui| {
        ui.monospace(name.to_uppercase());
        ui.monospace(format!("{:04X}", value));
      });
    }

    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {      
      {
        let mut temp = false;
        ui.checkbox(
          if crashed { &mut temp } else { &mut gb.running }, 
          "Running"
        );
      }
      egui::CollapsingHeader::new(
        "Registers"
      ).default_open(true).show(ui, |ui| {
        ui.horizontal(|ui| {
          register_view(ui, "af", gb.cpu.reg.af());
          register_view(ui, "bc", gb.cpu.reg.bc());
        });
        ui.horizontal(|ui| {
          register_view(ui, "de", gb.cpu.reg.de());
          register_view(ui, "hl", gb.cpu.reg.hl());
        });
        ui.horizontal(|ui| {
          register_view(ui, "sp", gb.cpu.reg.sp());
          register_view(ui, "pc", gb.cpu.reg.pc());
        });
      });
    });

    return exit;
  }
}
