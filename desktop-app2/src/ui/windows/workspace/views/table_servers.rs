use eframe::egui;

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
            // ── Header ──
            ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Name").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("IP Address").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("IP Address").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("SSH User").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Key Path").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                ui.label(egui::RichText::new("Status").size(12.0).color(window.style.color_gray_muted).strong())
            });
            ui.end_row();

            // ── Rows ──
            for server in &window.servers {
                ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(&server.server_name).size(13.0).color(window.style.color_white_cold))
                });
                ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(&server.server_ip).size(13.0).color(window.style.color_white_cold))
                });
                ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(&server.ssh_user).size(13.0).color(window.style.color_white_cold))
                });
                
                let key_display = if server.private_key_path.len() > 30 {
                    format!("...{}", &server.private_key_path[server.private_key_path.len()-27..])
                } else {
                    server.private_key_path.clone()
                };
                ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(key_display).size(12.0).color(window.style.color_gray_muted))
                });
                
                let (status_text, status_color) = if server.use_passphrase {
                    ("🔑 Secured", egui::Color32::from_rgb(80, 200, 80))
                } else {
                    ("🔓 No key", egui::Color32::from_rgb(180, 180, 60))
                };
                ui.add_sized([120.0, 20.0], |ui: &mut egui::Ui| {
                    ui.label(egui::RichText::new(status_text).size(12.0).color(status_color))
                });
                ui.end_row();
            }
        });

}