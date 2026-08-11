use bevy::ecs::system::{Query, ResMut};
use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;

use crate::visualizer;

pub fn update(
    mut contexts: EguiContexts,
    mut servers_query: Query<&mut visualizer::component::server::Server>,
    mut filter_state: ResMut<visualizer::resource::server_filter_state::ServerFilterState>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    const PANEL_WIDTH: f32 = 220.0;
    const PANEL_HEIGHT: f32 = 100.0;
    const PANEL_HEIGHT_MINIMIZED: f32 = 20.0;

    egui::Area::new("ServerFilters".into())
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::RIGHT_TOP, [0.0, 0.0])
        .show(ctx, |ui| {
            egui::Frame::window(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(5, 8, 14, 204))
                .show(ui, |ui| {
                    ui.set_min_size(egui::vec2(
                        PANEL_WIDTH, 
                        if filter_state.minimized{PANEL_HEIGHT_MINIMIZED}else{PANEL_HEIGHT}
                    ));
                    ui.set_max_width(PANEL_WIDTH);
                    ui.set_width(PANEL_WIDTH);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Show Server Traffic")
                                .color(egui::Color32::from_rgb(54, 200, 255)), // cian brillos
                        );

                        // Empuja el botón a la derecha
                        ui.with_layout(
                            egui::Layout::right_to_left(egui::Align::Center),
                            |ui| {
                                let button_text = if filter_state.minimized { "+" } else { "_" };

                                if ui.button(button_text).clicked() {
                                    filter_state.minimized = !filter_state.minimized;
                                }
                            },
                        );
                    });

                    ui.separator();

                    if !filter_state.minimized {
                        egui::ScrollArea::vertical()
                            .max_height(180.0)
                            .auto_shrink([false, true]) 
                            .show(ui, |ui| {
                                ui.set_width(PANEL_WIDTH);

                                let mut servers: Vec<Mut<visualizer::component::server::Server>> =
                                    servers_query.iter_mut().collect();

                                servers.sort_by(|a, b| {
                                    a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                });
                                for server in servers.iter_mut() {
                                    let name = server.name.clone(); 
                                    ui.checkbox(&mut server.visible, &name);
                                }
                            });
                    }
                });
        });
}
