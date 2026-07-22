use bevy::prelude::*;
use crate::{core::server::manager,visualizer::my_world::{self}};

pub fn run(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_data: ResMut<my_world::global::resources::WorldData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut cell_query: Query<&mut my_world::cell::components::Cell>,
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

        my_world::server::entity::create(
            &mut commands, &asset_server, &cell, server, &mut materials
        );
        
        commands.entity(cell_entity).insert(my_world::cell::components::CellState::Assigned);

        world_data.server_entity_map.insert(server.uuid.unwrap(), cell_entity);
       
        let sender = world_data.server_event_sender.clone();
        let server_threat = server.clone();
        commands.entity(cell_entity).insert(my_world::cell::components::CellState::Processing);
        std::thread::spawn(move || {
            server_threat.async_test_ssh_connection(sender);
        });
    }


}
