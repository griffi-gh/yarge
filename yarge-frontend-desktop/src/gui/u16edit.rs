use super::egui;
use egui::{RichText, TextStyle};

pub fn u16_edit(ui: &mut egui::Ui, name: &str, value: u16, allow_edit: bool, mul: u16) -> Option<u16> {
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
      let mut value_str = format!("{:X}", value);
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
          value_str = value_str.replace('0', "");
        }
        let x = u16::from_str_radix(
          ("0".to_string() + value_str.trim()).as_str(), 
          16
        );
        if let Ok(x) = x {
          ret = Some(x);
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
