use std::any::Any;

use eframe::egui;

#[derive(PartialEq, Clone, Debug)]
pub enum ProcessState {
    Idle, Running, Done, ProcessError(String), ValidateError(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum ExecutionState {Message(String), Done, Error(String),}

pub fn load_favicon() -> egui::IconData {
    let bytes = include_bytes!("../../assets/images/isotipo.png");
    let color_image = egui_extras::image::load_image_bytes(bytes)
        .expect("Failed to decode favicon");
    egui::IconData {
        rgba: color_image.pixels.iter().flat_map(|c| c.to_array()).collect(),
        width: color_image.width() as u32,
        height: color_image.height() as u32,
    }
}

pub fn load_texture(ctx: &egui::Context, bytes: &[u8]) -> egui::TextureHandle {
    let color_image = egui_extras::image::load_image_bytes(bytes)
        .expect("Failed to decode image");
    ctx.load_texture("name", color_image, egui::TextureOptions::LINEAR)
}

pub fn parse_error(e: Box<dyn Any + Send>) -> String {
    let msg = if let Some(s) = e.downcast_ref::<&str>() {
        let full = s.to_string();
        full.split(':').next().unwrap_or(&full).trim().to_string()
    } else if let Some(s) = e.downcast_ref::<String>() {
        let full = s.clone();
        full.split(':').next().unwrap_or(&full).trim().to_string()
    } else {
        "Unknown error".to_string()
    };
    msg
}