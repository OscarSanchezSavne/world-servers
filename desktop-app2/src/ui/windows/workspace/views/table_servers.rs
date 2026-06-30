use eframe::egui::{self, Align};

use crate::ui::windows::workspace::workspace_window::WorkspaceWindow;

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    ui.vertical(|ui| {
        ui.add_space(0.0);
    });
    egui::Grid::new("servers_grid")
        .striped(false)
        .min_col_width(150.0)
        .max_col_width(300.0)
        .spacing([20.0, 4.0])
        .show(ui, |ui| {
            let column_widths: Vec<[f32; 2]> = vec![
                [180.0, 20.0],
                [120.0, 20.0],
                [120.0, 20.0],
                [128.0, 20.0],
            ];
            
            // ── Header ──
            ui.add_sized(column_widths[0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Name").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized(column_widths[1], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Address").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized(column_widths[2], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("User").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized(column_widths[3], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Login type").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.end_row();

            // ── Rows ──
            for server in &window.servers {
                ui.add_sized(column_widths[0], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(&server.server_name).size(13.0).color(window.style.color_white_cold))
                });
                ui.add_sized(column_widths[1], |ui: &mut egui::Ui| {
                    ui.with_layout(egui::Layout::left_to_right(Align::Center), |ui| {
                        ui.label(egui::RichText::new(
                            format!("{}:{}", &server.server_ip, &server.server_port)
                        )
                        .size(13.0)
                        .color(window.style.color_white_cold));
                    }).response
                });
                ui.add_sized(column_widths[2], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(&server.ssh_user).size(13.0).color(window.style.color_white_cold))
                });
                
                let (status_text, status_color) = if server.use_password {
                    ("SSH", egui::Color32::from_rgb(80, 200, 80))
                } else {
                    ("Password", egui::Color32::from_rgb(180, 180, 60))
                };
                ui.add_sized(column_widths[3], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(status_text).size(12.0).color(status_color))
                });
                ui.end_row();
            }
        });

}