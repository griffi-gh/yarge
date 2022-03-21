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
  gb: Arc<Mutex<Gameboy>>
}
impl GuiState {
  pub fn new(gb: Arc<Mutex<Gameboy>>) -> Self {
    Self { gb }
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
    match self.gb.lock() {
      Ok(gb) => {
        egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {
          ui.label("Registers");
          ui.horizontal_wrapped(|ui| {
            ui.label(format!("PC: {:04X}", gb.cpu.reg.pc));
          });
        });
      }
      Err(err) => {
        egui::Window::new("Error").show(ui, |ui| {
          ui.label(
            RichText::new(format!(
              "{} has crashed",
              NAME.unwrap_or("The emulator")
            ))
            .color(Color32::from_rgb(0xff, 0x00, 0x00))
            .heading()
          );
          ui.collapsing("Details", |ui| {
            ui.label(format!("{}", err));
            if let Some(source) = err.source() {
              ui.separator();
              ui.label(format!("Caused by: {}", source));
            }
          });
        });
      }
    }
  }
}
