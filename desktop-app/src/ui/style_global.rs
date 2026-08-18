use eframe::egui::{self, Response, Vec2};

pub const LOGOTIPO_BYTES: &[u8] = include_bytes!("../../AppDir/usr/share/worldservers/assets/images/logotipo.png");
pub const ISOTIPO_BYTES: &[u8] = include_bytes!("../../AppDir/usr/share/worldservers/assets/images/isotipo.png");

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
            line_space: 12.0
        }
    }

    pub fn text_input_with_hint(
        &self, ui: &mut egui::Ui, label: &str, value: &mut String, placeholder: &str
    ) -> Response {
        self.label(ui, label);
        ui.add_sized(
    [ui.available_width(), 28.0],
        egui::TextEdit::singleline(value)
                .hint_text(placeholder) 
                .font(egui::TextStyle::Body)
                .text_color(self.input_text)
                .desired_width(f32::INFINITY)
                .margin(self.input_margin),
        )
        .on_hover_cursor(egui::CursorIcon::Text)
    }

    pub fn label(&self, ui: &mut egui::Ui, label: &str){
        ui.label(egui::RichText::new(label).color(self.color_gray_muted).size(13.0));
    }

    pub fn line_space(&self, ui: &mut egui::Ui) {
        ui.add_space(self.line_space);
    }

    pub fn button(&self, ui: &mut egui::Ui, label: &str, percentage_width: f32) -> Response {
        let btn_height = 28.0;
        let btn_width = (ui.available_width() - 8.0) / percentage_width; 

        ui.add_sized([btn_width, btn_height],
            egui::Button::new(egui::RichText::new(label)
                .color(self.color_white_cold))
                .fill(self.color_accent_mid)
                .corner_radius(4.0)
        )
    }

    pub fn button_orange(&self, ui: &mut egui::Ui, label: &str, percentage_width: f32) -> Response {
        let btn_height = 28.0;
        let btn_width = (ui.available_width() - 8.0) / percentage_width;

        ui.add_sized([btn_width, btn_height],
            egui::Button::new(
                egui::RichText::new(label).color(
                    egui::Color32::from_rgb(255, 240, 220)
                )
            )
                .fill(egui::Color32::from_rgb(230, 126, 34)) // naranja
                .corner_radius(4.0)
        )
    }

    pub fn info_panel(&self, ui: &mut egui::Ui, title: &str, label: &str) {
        
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(10, 20, 45))
            .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(50, 100, 200)))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    egui::RichText::new(title)
                        .size(13.0)
                        .color(egui::Color32::from_rgb(100, 180, 255))
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(label)
                        .size(12.0)
                        .color(egui::Color32::from_rgb(170, 210, 255)),
                );
            });
    }


    pub fn error_panel(&self, ui: &mut egui::Ui, error: &str) {
        egui::Frame::NONE
            .fill(egui::Color32::from_rgb(40, 10, 10))
            .stroke(egui::Stroke::new(1.0_f32, egui::Color32::from_rgb(180, 50, 50)))
            .corner_radius(6)
            .inner_margin(egui::Margin::symmetric(12, 8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.label(
                    egui::RichText::new("Ooops, something went wrong:")
                        .size(13.0)
                        .color(egui::Color32::from_rgb(230, 80, 80))
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(format!(" \u{2022} {}", error))
                        .size(12.0)
                        .color(egui::Color32::from_rgb(255, 150, 150)),
                );
            });
        self.line_space(ui);
    }


    pub fn log_panel(&self, ui: &mut egui::Ui, logs: &[String], scroll_to_cursor: bool) {

        egui::ScrollArea::vertical()
            .max_height(ui.available_height())
            .auto_shrink([false, false]) // que no se encoja
            .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::AlwaysVisible) // siempre visible
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        for log in logs {
                            ui.label(
                                egui::RichText::new(
                                    format!("\u{2022} {}", log)
                                )
                                    .color(egui::Color32::from_rgb(0, 255, 0))
                                    .size(12.0)
                                    .monospace()
                            );
                        }
                    });
                });
                if scroll_to_cursor{
                    ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                }
            });
    }

}
