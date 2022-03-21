use framework::{egui, Gui, InitProperties, Dimensions as Dim};
use egui::{Context, RichText, Color32};
use std::{
  sync::{Mutex, Arc},
  error::Error
};
use super::{gb::Gameboy, NAME}; //TODO get rid of dependency on gb

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 2;

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
  fn gui(&mut self, ui: &Context, _dim: Dim<f32>) {
    fn error_window(ui: &Context, title: &str, color: Color32, details: &str) {
      egui::TopBottomPanel::new(egui::panel::TopBottomSide::Top, "error_panel").show(ui, |ui| {
        ui.vertical_centered(|ui| {
          ui.label(RichText::new(title).color(color).size(18.));
        });
        ui.collapsing("Details", |ui| {
          ui.label(details);
          ui.label("Check console output for more details");
        });
      });
    }
    let gb = match self.gb.lock() {
      Ok(gb) => { gb },
      Err(err) => {
        let mut err_info = format!("{}", err);
        if let Some(source) = err.source() {
          err_info += format!("\nCaused by: {}", source).as_str();
        }
        error_window(
          &ui,
          format!(
            "{} thread panicked",
            NAME.unwrap_or("emulator")
          ).as_str(),
          Color32::RED,
          err_info.as_str()
        );
        err.into_inner()
      }
    };
    /*if let Some(err) = gb.error {
      error_window(
        &ui,
        format!(
          "{} crashed",
          NAME.unwrap_or("emulator")
        ).as_str(),
        Color32::YELLOW,
        "TODO"
      );
    }*/
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {
      ui.label("Registers");
      ui.horizontal_wrapped(|ui| {
        ui.label(format!("PC: {:04X}", gb.cpu.reg.pc));
      });
    });
  }
}
