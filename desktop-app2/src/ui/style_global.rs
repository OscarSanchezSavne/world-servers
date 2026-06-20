use eframe::egui;

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
        }
    }

}
