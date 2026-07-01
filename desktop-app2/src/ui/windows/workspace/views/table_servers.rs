use eframe::egui::{self, Align, Layout};
use egui_extras::{Column, TableBuilder};

use crate::ui::windows::workspace::workspace_window::WorkspaceWindow;

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
){
    ui.vertical(|ui| {
        ui.add_space(0.2);
    });

    TableBuilder::new(ui)
        .striped(false)
        .cell_layout(Layout::left_to_right(Align::Center))
        .columns(Column::initial(220.0), 1)
        .columns(Column::initial(180.0), 1)
        .columns(Column::initial(160.0), 1)
        .columns(Column::initial(188.0), 1)
        .columns(Column::initial(178.0), 1)
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.add_space(8.0);
                ui.label(egui::RichText::new("Name").size(12.0).color(window.style.color_gray_muted).strong());
            });
            header.col(|ui| {
                ui.label(egui::RichText::new("Address").size(12.0).color(window.style.color_gray_muted).strong());
            });
            header.col(|ui| {
                ui.label(egui::RichText::new("User").size(12.0).color(window.style.color_gray_muted).strong());
            });
            header.col(|ui| {
                ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::RightToLeft), |ui| {
                    ui.label(egui::RichText::new("Login type").size(12.0).color(window.style.color_gray_muted).strong());
                });
            });
            header.col(|_| {});
        })
        .body(|mut body| {
            for server in &window.servers.clone() {
                body.row(24.0, |mut row| {
                    row.col(|ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new(&server.server_name).size(13.0).color(window.style.color_white_cold));
                    });
                    row.col(|ui| {
                        ui.label(egui::RichText::new(
                            format!("{}:{}", &server.server_ip, &server.server_port)
                        )
                        .size(13.0)
                        .color(window.style.color_white_cold));
                    });
                    row.col(|ui| {
                        ui.label(egui::RichText::new(&server.ssh_user).size(13.0).color(window.style.color_white_cold));
                    });
                    row.col(|ui| {
                        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::RightToLeft), |ui| {
                            let (status_text, status_color) = if server.use_password {
                                ("Password", egui::Color32::from_rgb(80, 200, 80))
                            } else {
                                ("SSH", egui::Color32::from_rgb(180, 180, 60))
                            };
                            ui.label(egui::RichText::new(status_text).size(12.0).color(status_color));
                        });
                    });
                    row.col(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            egui::ComboBox::from_id_salt(format!("actions_{}", server.server_ip))
                                .selected_text(" Actions ")
                                .width(100.0)
                                .show_ui(ui, |ui| {
                                    ui.set_min_width(120.0);
                                    if ui.button("Edit").clicked() { }
                                    if ui.button("Test connection").clicked() {
                                        window.test_server(server.clone());
                                    }
                                    if ui.button("Delete").clicked() {
                                        window.delete_server(server);
                                    }
                                });
                        });
                        ui.add_space(8.0);
                    });
                });
            }
        });
}