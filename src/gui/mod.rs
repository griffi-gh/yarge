use egui::Context;
mod framework;
use framework::InitProperties;

const WIDTH: u32 = 160;
const HEIGHT: u32 = 144;
const SCALE: u32 = 2;

pub struct GuiState {
  age: u32,
  name: String,
}
impl Default for GuiState {
  fn default() -> Self{
    Self {
      age: 3,
      name: "fuck".to_string()
    }
  }
}
impl framework::Gui for GuiState {
  fn gui(&mut self, ui: &Context) {
    egui::Window::new(framework::PKG_NAME.unwrap_or("Debug")).show(ui, |ui| {
      ui.heading("My egui Application");
    });
    egui::SidePanel::left("side_panel").show(ui, |ui| {
      
      ui.horizontal(|ui| {
        ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name);
      });
      ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
      if ui.button("Click each year").clicked() {
        self.age += 1;
      }
      ui.label(format!("Hello '{}', age {}", self.name, self.age));
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
      min_size: (WIDTH, HEIGHT),
      size: (WIDTH * SCALE, HEIGHT * SCALE)
    });
  }
}
