use bevy::ecs::{entity::{Entity}, system::{Commands, Query, ResMut}};
use bevy_egui::{egui, EguiContexts};
use uuid::Uuid;

use crate::{ui::utilities::ExecutionState, visualizer};

pub fn update(
    mut contexts: EguiContexts,
    mut log_lines: ResMut<visualizer::resource::log_buffer::LogBuffer>,
) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };

    const LOG_WIDTH: f32 = 400.0;
    const LOG_HEIGHT: f32 = 160.0;
    const LOG_MINIMIZED_HEIGHT: f32 = 20.0;

    egui::Area::new("LiveLog".into())
        .order(egui::Order::Foreground)
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
                        //ui.set_width(LOG_WIDTH);

                        ui.label(
                            egui::RichText::new("SSH connection log")
                                .color(egui::Color32::from_rgb(47, 128, 237))
                                .size(10.0),
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


pub fn handle_server_ssh_messages(
    mut log: ResMut<visualizer::resource::log_buffer::LogBuffer>,
    servers_list: Query<&visualizer::component::servers::Servers>,
    server_query: Query<Entity>,
    mut commands: Commands,
) 
{
    let mut messages = Vec::new();
    if let Ok(rx) = log.rx.lock() {
        let servers_list= servers_list.single().unwrap();
        let servers_list= servers_list.list.clone();
        
        let get_server= |server_uuid: Uuid|{
            servers_list.iter().find(
                |server| server.uuid == Some(server_uuid)
            ).unwrap().clone()
        };
        while let Ok(msg) = rx.try_recv() {
            match msg {
                ExecutionState::Message(server_uuid, text) => {
                    let server_model= get_server(server_uuid);
                    messages.push(format!("[{}] {}", server_model.name, text));
                },
                ExecutionState::Done(server_uuid) => {
                    let server_model= get_server(server_uuid);
                    if let Ok(server) = server_query.get(server_model.entity.unwrap()) {
                        commands.entity(server).insert(
                            visualizer::component::server::ServerState::Inline
                        );
                    }
                    messages.push(format!("[{}] Inline", server_model.name));
                    break;
                }
                ExecutionState::Error(server_uuid, e) => {
                    let server_model= get_server(server_uuid);
                    if let Ok(server) = server_query.get(server_model.entity.unwrap()) {
                        commands.entity(server).insert(
                            visualizer::component::server::ServerState::Error
                        );
                    }
                    messages.push(format!("[{}] ERROR : {}", server_model.name, e));
                    break;
                }
            }
        }
    }
    for msg in messages {
        log.push(msg);
    }

}