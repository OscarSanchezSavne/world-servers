use bevy::prelude::*;
use crate::{core::server::manager,visualizer::my_world0::{self}};

pub fn run(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_data: ResMut<my_world0::global::resources::WorldData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cell_query: Query<&mut my_world0::cell::components::Cell>,
) 
{
    let servers= manager::Server::get_servers();
    world_data.servers= servers.clone();
    for server in servers.iter()
    {
        let position= world_data.index_cells_sort.pop().unwrap();
        let cell_entity= world_data.cells[position];
        let mut cell = cell_query.get_mut(cell_entity).unwrap();
        cell.external= false;

        my_world0::server::entity::create(
            &mut commands, &asset_server, &cell, server, &mut materials, &mut world_data
        );
        
        commands.entity(cell_entity).insert(
            my_world0::cell::components::CellState::Assigned
        );

        commands.entity(cell_entity).insert(
            my_world0::server::components::ServerModel{
                uuid: server.uuid.unwrap()
            }
        );

        world_data.server_cell_entity_map.insert(server.uuid.unwrap(), cell_entity);
       
        let sender = world_data.server_event_sender.clone();
        let server_threat = server.clone();
        std::thread::spawn(move || {
            server_threat.async_test_ssh_connection(sender);
        });
    }


}
