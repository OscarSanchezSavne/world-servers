use bevy::{ecs::system::{Commands, Query, Res, ResMut}, math::Vec3, transform::components::Transform};

use crate::{ui::utilities::{ExecutionState, ServerTraffic}, visualizer::my_world0::{self, packet_sphere::resources::SphereAssets}};


pub fn server_ready(
    world_data: Res<my_world0::global::resources::WorldData>,
    mut commands: Commands,
    mut log: ResMut<my_world0::log::resource::LogBuffer>
) {
    let receiver = world_data.server_event_receiver.lock().unwrap();
    while let Ok(msg) = receiver.try_recv() {
        match msg {
            ExecutionState::Message(server_uuid, text) => {
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] {}", server_name, text));
            },
            ExecutionState::Done(server_uuid) => {
                if let Some(cell_entity) = world_data.server_cell_entity_map.get(&server_uuid) {
                    commands.entity(cell_entity.clone()).insert(my_world0::cell::components::CellState::InLine);
                }
                
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] Inline", server_name));
                break;
            }
            ExecutionState::Error(server_uuid, e) => {
                if let Some(cell_entity) = world_data.server_cell_entity_map.get(&server_uuid) {
                    commands.entity(cell_entity.clone()).insert(my_world0::cell::components::CellState::Failed);
                }
                
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] ERROR : {}", server_name, e));
                break;
            }
        }
    }
}

pub fn server_traffic(
    world_data: Res<my_world0::global::resources::WorldData>,
    mut commands: Commands,
    traffic_channel: ResMut<my_world0::server::resources::TrafficChannel>,
    sphere_assets: Res<SphereAssets>,
    query_transform: Query<&Transform>,  
) {
    let rx = traffic_channel.rx.lock().unwrap();
    while let Ok(msg) = rx.try_recv() {
        match msg {
            ServerTraffic::Package(server_uuid, package) => {
                let server_entity= world_data.server_entity_map
                    .get(&server_uuid).unwrap();
                
                 let source_pos = query_transform
                    .get(*server_entity)
                    .map(|t| t.translation)
                    .unwrap_or(Vec3::ZERO);

                    
                my_world0::packet_sphere::entity::create(
                    &mut commands, package.clone(), 
                    source_pos, 
                    &sphere_assets
                );
                dbg!(package);
            },
            ServerTraffic::Error(server_uuid, e) => {
                /*if let Some(cell_entity) = world_data.server_cell_entity_map.get(&server_uuid) {
                    commands.entity(cell_entity.clone()).insert(my_world::cell::components::CellState::Failed);
                }
                
                let server_name = world_data.get_server(server_uuid).server_name;
                log.push(format!("[{}] ERROR : {}", server_name, e));
                break;*/
            }
        }
    }
}
