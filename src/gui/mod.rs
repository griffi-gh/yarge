use egui::Context;
mod framework;
use framework::InitProperties;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 2;

pub struct GuiState {
  
}
impl Default for GuiState {
  fn default() -> Self{
    Self {
      
    }
  }
}
impl framework::Gui for GuiState {
  fn gui(&mut self, ui: &Context) {
    egui::Window::new(framework::PKG_NAME.unwrap_or("Debug")).show(ui, |ui| {
      ui.label("My egui Application");
    });
  }
}
impl GuiState {
  pub fn new() -> Self {
    Self::default()
  }
  ///Warning: consumes self!
  pub fn init(self) {
    framework::init(Box::new(self), InitProperties {
      title: framework::PKG_NAME.unwrap_or("open source gameboy emulator"),
      pixels_resoltion: (WIDTH, HEIGHT),
      min_size: (WIDTH, HEIGHT),
      size: (WIDTH * SCALE, HEIGHT * SCALE),
    });
  }
}
