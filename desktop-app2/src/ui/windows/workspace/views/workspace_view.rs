use eframe::egui::{self};
use egui::panel::Panel;

use crate::ui::windows::workspace::workspace_window::WorkspaceWindow;

pub fn render(
    ui: &mut egui::Ui, window: &mut WorkspaceWindow
) {
    // ── Top bar ──
    Panel::top("top_bar")
        .frame(egui::Frame {
            fill: window.style.color_panel_bg,
            inner_margin: egui::Margin::symmetric(12, 8),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::from_texture(&window.logotipo).max_height(28.0).max_width(220.0)
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let btn = egui::Button::new(
                        egui::RichText::new("Register server").color(window.style.color_white_cold),
                    )
                    .fill(window.style.color_accent_mid)
                    .stroke(egui::Stroke::NONE)
                    .corner_radius(4.0);
                    if ui.add(btn).clicked() {
                        // action
                    }
                });
            });
        });

    // ── Footer ──
    Panel::bottom("footer")
        .frame(egui::Frame {
            fill: window.style.color_panel_bg,
            inner_margin: egui::Margin::symmetric(12, 4),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new(format!("{}:{}", window.state.central_host, window.state.central_port) )
                        .size(11.0)
                        .color(window.style.color_gray_muted),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(
                        egui::RichText::new("EN")
                            .size(11.0)
                            .color(window.style.color_cyan_highlight),
                    ).clicked() {
                        // toggle language
                    }
                    ui.label(
                        egui::RichText::new("Language:")
                            .size(11.0)
                            .color(window.style.color_gray_muted),
                    );
                    ui.add_space(8.0);
                    if ui.button(
                        egui::RichText::new("Setup")
                            .size(11.0)
                            .color(window.style.color_cyan_highlight),
                    ).clicked() {
                        window.state.show_setup = true;
                    }
                });
            });
        });

    // ── Central panel ──
    egui::CentralPanel::default()
        .frame(egui::Frame {
            fill: window.style.color_bg_deep,
            inner_margin: egui::Margin::symmetric(12, 0),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            let card_frame = egui::Frame {
                fill: window.style.color_panel_bg,
                corner_radius: egui::CornerRadius::same(12),
                stroke: egui::Stroke::new(1.0, window.style.color_card_stroke),
                shadow: egui::epaint::Shadow {
                    offset: [0, 2].into(),
                    blur: 12,
                    spread: 0,
                    color: egui::Color32::from_black_alpha(60),
                },
                ..Default::default()
            };
            ui.add_space(24.0);

            // ── Card 1: Welcome ──
            card_frame.show(ui, |ui| {
                ui.add_space(16.0);
                ui.vertical_centered(|ui| {
                    ui.add(
                        egui::Image::from_texture(&window.isotipo)
                            .max_width(64.0)
                            .max_height(64.0),
                    );
                    ui.add_space(12.0);
                    ui.add(egui::Label::new(
                        egui::RichText::new("Welcome to WorldServers")
                            .size(28.0)
                            .color(window.style.color_white_cold)
                            .strong(),
                    ));
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Manage your remote servers in real time")
                            .size(14.0)
                            .color(window.style.color_gray_muted),
                    );
                });
                ui.add_space(16.0);
            });

            ui.add_space(24.0);

            // ── Card 2: Server table ──
            card_frame.show(ui, |ui| {
                ui.add_space(8.0);
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // table headers + rows
                    egui::Grid::new("servers_grid")
                        .striped(true)
                        .min_col_width(0.0)
                        .show(ui, |ui| {
                            // headers...
                            // rows...
                        });
                    ui.add_space(16.0);
                });
                ui.add_space(8.0);
            });
        });

    if window.state.show_setup {
        let mut open = true;
        egui::Window::new("Network Setup")
            .open(&mut open)
            .collapsible(false)
            .resizable(false)
            .fixed_size([560.0, 840.0])
            .show(ui.ctx(), |ui| {
                // campos: IP, puerto, validaciones, botón guardar
            });
        if !open {
            window.state.show_setup = false;
        }
    }

}