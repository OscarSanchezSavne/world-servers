use eframe::{egui::{self}};
use egui::panel::Panel;

use crate::ui::windows::workspace::{views::{modal_server_form_view, modal_setup_view, table_servers}, workspace_window::WorkspaceWindow};

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
                    if window.style.button(
                        ui, "Register Server", 3.5
                    ).clicked() {
                        window.open_server_form();
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
                    egui::RichText::new(
                        format!("{}:{}", window.setup.central_host, window.setup.central_port) 
                    )
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
                        window.setup_state.show_setup = true;
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

                if window.servers.is_empty() {
                    ui.add_space(8.0);
                    ui.vertical_centered(|ui| {
                        ui.add_space(60.0);
                        ui.label(
                            egui::RichText::new("📡  No servers registered yet")
                                .size(18.0)
                                .color(window.style.color_gray_muted)
                                .strong(),
                        );
                        ui.add_space(8.0);

                        window.style.info_panel(ui, 
                            "ℹ️  Getting started",
                            "To begin, you must register at least one server. Click 'Register server' in the top bar to add your first server."
                        );

                        ui.add_space(60.0);
                    });
                } else {
                    table_servers::render(ui, window);
                }

                ui.add_space(8.0);
            });

        });

    if window.setup_state.show_setup {
        modal_setup_view::render(ui, window);
    }

    if window.server_form.show {
        modal_server_form_view::render(ui, window);
    }

}