use framework::{egui, Gui, InitProperties};
use std::sync::{Mutex, Arc};
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
  fn gui(&mut self, ui: &egui::Context) {
    let gb = self.gb.lock().unwrap();
    egui::Window::new(NAME.unwrap_or("debug")).show(ui, |ui| {
      ui.label("Registers");
      ui.horizontal_wrapped(|ui| {
        ui.label(format!("PC: {:02X}", gb.cpu.reg.pc));
      });
    });
  }
}
