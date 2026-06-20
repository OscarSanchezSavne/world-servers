use eframe::egui;

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