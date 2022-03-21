use framework::{egui, Gui, InitProperties};
use egui::{Context, RichText, Color32};
use std::{
  sync::{Mutex, Arc},
  error::Error
};
use super::gb::Gameboy; //TODO get rid of dependency on gb

const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 2;

pub struct GuiState {
  gb: Arc<Mutex<Gameboy>>,
  error_window_open: bool,
}
impl GuiState {
  pub fn new(gb: Arc<Mutex<Gameboy>>) -> Self {
    Self {
      gb,
      error_window_open: true,
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
  fn gui(&mut self, ui: &Context) {
    let gb = match self.gb.lock() {
      Ok(gb) => {
        self.error_window_open = true;
        gb
      },
      Err(err) => {
        egui::Window::new(RichText::new("Error"))
          .collapsible(false)
          .open(&mut self.error_window_open)
          .show(ui, |ui| {
            ui.label(
              RichText::new(format!(
                "{} has panicked",
                NAME.unwrap_or("the emulator")
              ))
              .color(Color32::from_rgb(0xff, 0x00, 0x00))
              .size(18.)
            );
            ui.collapsing("Details", |ui| {
              ui.label(format!("{}", err));
              if let Some(source) = err.source() {
                ui.label(format!("Caused by: {}", source));
              }
              ui.label("Check console output for more details");
            });
          });
        err.into_inner()
      }
    };
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {
      ui.label("Registers");
      ui.horizontal_wrapped(|ui| {
        ui.label(format!("PC: {:04X}", gb.cpu.reg.pc));
      });
    });
  }
}
