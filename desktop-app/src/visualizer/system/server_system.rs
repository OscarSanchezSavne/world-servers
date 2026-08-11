use bevy::{ecs::{query::{Added, Changed, Or}, system::Query}};
use bevy::prelude::*;

use crate::{ui::{self, utilities::ServerTraffic::Package}, visualizer};

pub fn create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut visualizer::component::grid::Grid>,
    mut servers_query: Query<
        &mut visualizer::component::servers::Servers, 
        Or<(Added<visualizer::component::servers::Servers>, Changed<visualizer::component::servers::Servers>)>
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
        server.entity= Some(visualizer::entity::server_entity::create(
            &mut commands, &asset_server, cell.clone(), server
        ));
    }

}

pub fn attach_cell_entity(
    mut commands: Commands,
    mut servers_query: Query<(Entity, &mut visualizer::component::server::Server, &visualizer::component::cell::Cell)>,
    cells_query: Query<(Entity, &visualizer::component::cell::Cell), Without<visualizer::component::server::Server>>,
) 
{
    for (server_entity, mut server, server_cell) in servers_query.iter_mut() {
        if server.entity_cell != None{
            continue;
        }
        if let Some(cell_entity) = cells_query.iter().find(|(_, cell)| cell.uuid == server_cell.uuid).map(|(e, _)| e) {
            server.entity_cell = Some(cell_entity);
            if server.external {
                commands.entity(cell_entity).insert(visualizer::component::cell::CellType::External);
            } else {
                commands.entity(cell_entity).insert(visualizer::component::cell::CellType::Internal);
            }
        }
        commands.entity(server_entity).insert(
            visualizer::component::server::ServerReady::default()
        );
    }

}


pub fn connect(
    mut commands: Commands,
    server_list: Query<&visualizer::component::servers::Servers>,
    query_servers: Query<
        (
            Entity, &mut visualizer::component::server::Server
        ),
        Added<visualizer::component::server::ServerReady>,
    >,
    log: ResMut<visualizer::resource::log_buffer::LogBuffer>,
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
            visualizer::component::server::ServerState::Connecting
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
    mesh_query: Query< Entity, With<MeshMaterial3d<StandardMaterial>>, >,

    mut materials: ResMut<Assets<StandardMaterial>>,
    servers_query: Query<
        (&visualizer::component::server::Server, &visualizer::component::server::ServerState),
        Or<(Added<visualizer::component::server::ServerState>, Changed<visualizer::component::server::ServerState>)>,
    >,

    name_query: Query<&Name>,
) {
    for (server, state) in &servers_query {
        let Some(cell_entity) = server.entity_cell else {
            continue;
        };

        let (border_color, border_emissive) = match state {
            visualizer::component::server::ServerState::Inline => {
                commands
                    .entity(cell_entity)
                    .insert(visualizer::component::cell::PulsingCell::stop());
                (
                    // Verde oscuro
                    Color::srgb_u8(0x12, 0x78, 0x58),
                    // Glow verde
                    LinearRgba::rgb(0.03, 0.75, 0.32))
            }

            visualizer::component::server::ServerState::Error => {
                commands
                    .entity(cell_entity)
                    .insert(visualizer::component::cell::PulsingCell::stop());

                (
                    Color::srgb_u8(0x9A, 0x30, 0x38),
                    LinearRgba::rgb(0.90, 0.05, 0.06),
                )
            }
            visualizer::component::server::ServerState::Connecting => {
                commands
                    .entity(cell_entity)
                    .insert(visualizer::component::cell::PulsingCell::start());

                (
                    Color::srgb_u8(0x24, 0x58, 0x8A),
                    LinearRgba::rgb(0.04, 0.28, 0.95),
                )
            }
        };

        let border_material = materials.add(
            StandardMaterial {
                base_color: border_color,
                emissive: border_emissive,
                metallic: 0.0,
                perceptual_roughness: 0.70,
                ..default()
            }
        );

        for descendant in children.iter_descendants(cell_entity) {
            let Ok(name) = name_query.get(descendant) else {
                continue;
            };

            let name = name.as_str();

            if !name.starts_with("external_border") && name != "internal_border" { continue; }

            for child in children.iter_descendants(descendant) {
                if !mesh_query.contains(child) {
                    continue;
                }

                commands
                    .entity(child)
                    .insert(
                        MeshMaterial3d(
                            border_material.clone()
                        )
                    );
            }
        }
    }
}




pub fn capture_traffic(
    server_list: Query<&visualizer::component::servers::Servers>,
    servers_query: Query<
        (&visualizer::component::server::Server, &visualizer::component::server::ServerState), 
        Or<(Added<visualizer::component::server::ServerState>,Changed<visualizer::component::server::ServerState>)>,
    >,
    package_data_queue: Res<visualizer::resource::package_data_queue::PackageDataQueue>
){
    let Ok(original_servers) = server_list.single() else {
        return;
    };
    
    for (server_component, state) in servers_query {

        if *state != visualizer::component::server::ServerState::Inline{
            continue;
        }
        
        let Some(core_server) = original_servers
            .list_original_servers
            .iter()
            .find(|server| server.uuid == server_component.uuid)
            .cloned()
        else {
            continue;
        };

        let (tx, rx) = std::sync::mpsc::channel::<ui::utilities::ServerTraffic>();
        
        // Hilo 1: outbound
        let core1 = core_server.clone();
        let tx1 = tx.clone();
        std::thread::spawn(move || {
            core1.async_run_tcpdump(tx1, false);
        });

        // Hilo 2: inbound
        let tx2 = tx.clone();
        std::thread::spawn(move || {
            core_server.async_run_tcpdump(tx2, true);
        });

        // Un solo hilo de consumo (rx no se clona)
        let tx_packages = package_data_queue.tx.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Package(server_uuid, package) => {
                        tx_packages
                            .send(visualizer::component::package_data::RawPackage {
                                server_uuid,
                                package_data: package,
                            })
                            .unwrap();
                    }
                    _ => {}
                }
            }
        }); 

    }
 
}




