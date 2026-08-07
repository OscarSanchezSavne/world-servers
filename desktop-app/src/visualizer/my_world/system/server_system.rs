use bevy::{ecs::{query::{Added, Changed, Or}, system::Query}};
use bevy::prelude::*;

use crate::{ui, visualizer::my_world};

pub fn create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut my_world::component::grid::Grid>,
    mut servers_query: Query<
        &mut my_world::component::servers::Servers, 
        Or<(Added<my_world::component::servers::Servers>, Changed<my_world::component::servers::Servers>)>
    >,
) 
{
    let mut grid= grid_query.single_mut().unwrap();
    
    if servers_query.single().is_err() {
        return;
    }

    let mut servers= servers_query.single_mut().unwrap();

    for server in servers.list.iter_mut() {
        if server.entity != None {
            continue;   
        }
        let cell= grid.get_free_cell();
        server.entity= Some(my_world::entity::server_entity::create(
            &mut commands, &asset_server, cell.clone(), server.clone()
        ));
    }

}

pub fn attach_cell_entity(
    mut commands: Commands,
    mut servers_query: Query<(Entity, &mut my_world::component::server::Server, &my_world::component::cell::Cell)>,
    cells_query: Query<(Entity, &my_world::component::cell::Cell), Without<my_world::component::server::Server>>,
) 
{
    for (server_entity, mut server, server_cell) in servers_query.iter_mut() {
        if server.entity_cell != None{
            continue;
        }
        if let Some(cell_entity) = cells_query.iter().find(|(_, cell)| cell.uuid == server_cell.uuid).map(|(e, _)| e) {
            server.entity_cell = Some(cell_entity);
            if server.external {
                commands.entity(cell_entity).insert(my_world::component::cell::CellType::External);
            } else {
                commands.entity(cell_entity).insert(my_world::component::cell::CellType::Internal);
            }
        }
        commands.entity(server_entity).insert(
            my_world::component::server::ServerReady::default()
        );
    }

}


pub fn connect(
    mut commands: Commands,
    server_list: Query<&my_world::component::servers::Servers>,
    query_servers: Query<
        (
            Entity, &mut my_world::component::server::Server
        ),
        Added<my_world::component::server::ServerReady>,
    >,
    log: ResMut<my_world::resource::log_buffer::LogBuffer>,
) 
{
    let server_list= server_list.single().unwrap();
    let server_list= server_list.list_original_servers.clone();
    
    for (server_entity,server_component) in query_servers {
        if server_component.external == true {
            continue;
        }
        let core_server= server_list.iter().find(
            |server| server.uuid == server_component.uuid
        ).unwrap().clone();

        let (tx, rx) = std::sync::mpsc::channel::<ui::utilities::ExecutionState>();

        commands.entity(server_entity).insert(
            my_world::component::server::ServerState::Connecting
        );
        std::thread::spawn(move || {
            core_server.async_test_ssh_connection(tx);
        });

        let tx_log_thread= log.tx.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                tx_log_thread.send(msg).unwrap();
            }
        });

    }
}



pub fn refresh_connection_state(
    mut commands: Commands,
    children: Query<&Children>,
    mesh_query: Query<Entity, With<MeshMaterial3d<StandardMaterial>>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    servers_query: Query<
        (&my_world::component::server::Server, &my_world::component::server::ServerState), 
        Or<(Added<my_world::component::server::ServerState>,Changed<my_world::component::server::ServerState>)>,
    >,
    name_query: Query<&Name>
) {
    for (server, state) in servers_query {

        let cell_entity= server.entity_cell.unwrap();
        let color = match state {
            my_world::component::server::ServerState::Inline=>{
                commands.entity(cell_entity).insert(
                    my_world::component::cell::PulsingCell::stop()
                );
                Color::srgb(0.0, 0.5, 0.0)
            },
            my_world::component::server::ServerState::Error=>{
                commands.entity(cell_entity).insert(
                    my_world::component::cell::PulsingCell::stop()
                );
                Color::srgb(0.9, 0.0, 0.0)
            },
            my_world::component::server::ServerState::Connecting => {
                commands.entity(cell_entity).insert(
                    my_world::component::cell::PulsingCell::start()
                );
                Color::srgb(0.0, 0.0, 0.9)
            },
        };

        let material = materials.add(StandardMaterial {
            base_color: color,
            ..default()
        });

        for descendant in children.iter_descendants(cell_entity) {
            if let Ok(name) = name_query.get(descendant) {
                
                if !name.as_str().starts_with("external_floor") {
                    continue;
                }
                for child in children.iter_descendants(descendant) {
                    if mesh_query.contains(child) {
                        commands.entity(child).insert(MeshMaterial3d(material.clone()));
                    }
                }

            }
        }
    }
 
}




