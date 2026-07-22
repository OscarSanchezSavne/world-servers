use bevy::ecs::system::{Commands, Res, ResMut};

use crate::{ui::utilities::ExecutionState, visualizer::my_world};


pub fn server_ready(
    world_data: Res<my_world::global::resources::WorldData>,
    mut commands: Commands,
    mut log: ResMut<my_world::log::resource::LogBuffer>
) {
    let receiver = world_data.server_event_receiver.lock().unwrap();
    while let Ok(msg) = receiver.try_recv() {
        match msg {
            ExecutionState::Message(server_uuid, text) => {
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] {}", server_name, text));
            },
            ExecutionState::Done(server_uuid) => {
                if let Some(cell_entity) = world_data.server_entity_map.get(&server_uuid) {
                    commands.entity(cell_entity.clone()).insert(my_world::cell::components::CellState::InLine);
                }
                
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] Inline", server_name));
                break;
            }
            ExecutionState::Error(server_uuid, e) => {
                if let Some(cell_entity) = world_data.server_entity_map.get(&server_uuid) {
                    commands.entity(cell_entity.clone()).insert(my_world::cell::components::CellState::Failed);
                }
                
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] ERROR : {}", server_name, e));
                break;
            }
        }
    }
}
