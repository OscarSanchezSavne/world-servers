use eframe::egui::{self, Vec2};

pub const LOGOTIPO_BYTES: &[u8] = include_bytes!("../../../assets/images/logotipo.png");
pub const ISOTIPO_BYTES: &[u8] = include_bytes!("../../assets/images/isotipo.png");

pub struct StyleGlobal {
    pub color_bg_deep: egui::Color32,
    pub color_card_stroke: egui::Color32,
    pub color_panel_bg: egui::Color32,
    pub color_cyan_highlight: egui::Color32,
    pub color_white_cold: egui::Color32,
    pub color_accent_mid: egui::Color32,
    pub color_gray_muted: egui::Color32,

    pub input_margin: Vec2,
    pub input_text: egui::Color32,
    pub line_space: f32,
}


impl StyleGlobal {

    pub fn new() -> Self {
        Self{
            color_bg_deep: egui::Color32::from_rgb(5, 8, 14),
            color_card_stroke: egui::Color32::from_rgb(25, 35, 50),
            color_panel_bg: egui::Color32::from_rgb(11, 17, 27),
            color_cyan_highlight: egui::Color32::from_rgb(54, 200, 255),
            color_white_cold: egui::Color32::from_rgb(215, 220, 232),
            color_accent_mid: egui::Color32::from_rgb(15, 95, 234),
            color_gray_muted: egui::Color32::from_rgb(142, 149, 166),
            
            input_margin: egui::vec2(8.0, 6.0),
            input_text: egui::Color32::from_rgb(215, 220, 232),
            line_space: 12 as f32,
        }
    }

    pub fn text_input(&self, ui: &mut egui::Ui, label: &str, value: &mut String) -> bool {
        ui.label(egui::RichText::new(label).color(self.color_gray_muted).size(13.0));
        ui.add_sized(
    [ui.available_width(), 28.0],
        egui::TextEdit::singleline(value)
                .font(egui::TextStyle::Body)
                .text_color(self.input_text)
                .desired_width(f32::INFINITY)
                .margin(self.input_margin),
        )
        .on_hover_cursor(egui::CursorIcon::Text)
        .changed()
    }

    pub fn line_space(&self, ui: &mut egui::Ui) {
        ui.add_space(self.line_space);
    }


    pub fn title(&self, ui: &mut egui::Ui, label: &str) {
        ui.heading(
            egui::RichText::new(label)
                .color(self.color_white_cold)
                .size(18.0)
        );
        ui.add_space(16.0);
    }

    pub fn button(&self, ui: &mut egui::Ui, label: &str) -> bool {
        ui.add(
            egui::Button::new(
                egui::RichText::new(label)
                    .color(self.color_white_cold)
            )
            .fill(self.color_accent_mid)
            .corner_radius(4.0)
        ).clicked()
    }

}
