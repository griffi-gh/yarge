use egui::Context;
mod framework;

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
    egui::SidePanel::left("side_panel").show(ui, |ui| {
      ui.heading("My egui Application");
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

pub fn init() {
  framework::init(Box::new(GuiState::default()));
}
