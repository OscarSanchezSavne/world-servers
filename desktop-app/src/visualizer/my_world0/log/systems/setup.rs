use bevy::ecs::system::{ResMut};
use bevy_egui::{egui, EguiContexts};

use crate::visualizer::my_world0;

pub fn run(
    mut contexts: EguiContexts,
    mut log_lines: ResMut<my_world0::log::resource::LogBuffer>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    const LOG_WIDTH: f32 = 400.0;
    const LOG_HEIGHT: f32 = 160.0;
    const LOG_MINIMIZED_HEIGHT: f32 = 20.0;

    egui::Area::new("LiveLog".into())
        .anchor(egui::Align2::RIGHT_BOTTOM, [0.0, 0.0])
        .show(ctx, |ui| {
            egui::Frame::window(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(5, 8, 14, 204))
                .show(ui, |ui| {
                    let height = if log_lines.minimized {
                        LOG_MINIMIZED_HEIGHT
                    } else {
                        LOG_HEIGHT
                    };

                    // Fuerza mismo ancho siempre
                    ui.set_min_size(egui::vec2(LOG_WIDTH, height));
                    ui.set_max_width(LOG_WIDTH);
                    ui.set_width(LOG_WIDTH);

                    ui.horizontal(|ui| {
                        ui.set_width(LOG_WIDTH);

                        ui.label(
                            egui::RichText::new("Live Log")
                                .color(egui::Color32::GREEN),
                        );

                        // Empuja el botón a la derecha
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                let button_text = if log_lines.minimized { "+" } else { "_" };

                                if ui.button(button_text).clicked() {
                                    log_lines.minimized = !log_lines.minimized;
                                }
                            },
                        );
                    });

                    if !log_lines.minimized {
                        egui::ScrollArea::vertical()
                            .stick_to_bottom(true)
                            .max_height(140.0)
                            .show(ui, |ui| {
                                ui.set_width(LOG_WIDTH);

                                ui.with_layout(
                                    egui::Layout::top_down(egui::Align::LEFT),
                                    |ui| {
                                        for line in log_lines.iter() {
                                            if line.contains("ERROR "){
                                                ui.label(
                                                    egui::RichText::new(line).color(egui::Color32::DARK_RED),
                                                );
                                            }else{
                                                ui.label(
                                                    egui::RichText::new(line).color(egui::Color32::DARK_GREEN),
                                                );
                                            }
                                        }
                                    },
                                );
                            });
                    }
                });
        });
}