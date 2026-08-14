use bevy::{ecs::{query::{Added, Changed, Or}, system::Query}};
use bevy::prelude::*;
use bevy_fontmesh::TextMesh;

use crate::{ui::{self, utilities::{ServerMetrics, ServerTraffic::Package}}, visualizer};

pub fn create(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut grid_query: Query<&mut visualizer::component::grid::Grid>,
    mut servers_query: Query<
        &mut visualizer::component::servers::Servers, 
        Or<(Added<visualizer::component::servers::Servers>, Changed<visualizer::component::servers::Servers>)>
    >,
    mut materials: ResMut<Assets<StandardMaterial>>,
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
            &mut commands, &asset_server, cell.clone(), server, &mut materials
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
                commands
                    .entity(server.entity.unwrap())
                    .insert(visualizer::component::server::ServerReadyMetrics::default());
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

        let internal_servers= original_servers.list_original_servers.clone();
        std::thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Package(server_uuid, mut package) => {

                        let src_exists = internal_servers
                            .iter()
                            .any(|server| server.server_ip == package.src_ip);

                        let dst_exists = src_exists && internal_servers
                            .iter()
                            .any(|server| server.server_ip == package.dst_ip);

                        let internal = src_exists && dst_exists;
                        package.internal = Some(internal);

                        if internal && package.inbound {
                            continue;
                        }

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


pub fn start_capture_metrics(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,

    mut servers_query: Query<
        (Entity, &mut visualizer::component::server::Server), 
        Added<visualizer::component::server::ServerReadyMetrics>,
    >,
)
{
    let block= Cuboid::new(4.27, 1.0, 0.2);
    let line_h= Cuboid::new(4.4, 0.09, 0.3);
    let line_v= Cuboid::new(0.092, 1.09, 0.3);
            
    let material_border= materials.add(StandardMaterial {
            base_color: Color::srgb_u8(82, 103, 127),
            unlit: true,
            ..default()
        });

    let back_color_load_block= Color::srgb_u8(36, 50, 68);
    for (server_entity, mut server) in servers_query.iter_mut() {
        commands.entity(server_entity).with_children(|parent| {

            //CPU 
            parent.spawn((
                TextMesh{
                    text: "CPU".to_string(),
                    font: asset_server
                        .load("fonts/Fira_Sans/FiraSans-ExtraLight.ttf"),
                    ..default()
                },
                MeshMaterial3d(materials.add(StandardMaterial {
                    unlit: true,
                    base_color: Color::srgb_u8(180, 190, 205),
                    ..default()
                })),
                Transform::from_xyz(-5.5, -1.5, -0.3), // relativo al padre
            ));

            //Cargador
            server.entity_cpu_load= Some(parent.spawn((
                Mesh3d(meshes.add(block)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        unlit: true,
                        base_color: Color::srgb_u8(255, 50, 50),
                        ..default()
                    })
                ),
                Transform::from_translation(Vec3::new(-4.65, -3.22, -0.3)),   
                Visibility::Hidden, 
            )).id());
            parent.spawn((
                Mesh3d(meshes.add(block)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        unlit: true,
                        base_color: back_color_load_block,
                        ..default()
                    })
                ),
                Transform::from_translation(Vec3::new(-4.65, -3.22, -0.35)),   
            ));

            parent.spawn((
                Mesh3d(meshes.add(line_h)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-4.65, -2.72, -0.22)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_h)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-4.65, -3.72, -0.22)),   
            ));

            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-6.85, -3.22, -0.22)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-5.75, -3.22, -0.22)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-4.65, -3.22, -0.22)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-3.55, -3.22, -0.22)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-2.45, -3.22, -0.22)),   
            ));



            // DISK
            parent.spawn((
                TextMesh{
                    text: "Disk".to_string(),
                    font: asset_server
                        .load("fonts/Fira_Sans/FiraSans-ExtraLight.ttf"),
                    ..default()
                },
                MeshMaterial3d(materials.add(StandardMaterial {
                    unlit: true,
                    base_color: Color::srgb_u8(180, 190, 205),
                    ..default()
                })),
                Transform::from_xyz(-0.7, -4.5, -0.3), // relativo al padre
            ));

            //Cargador
            server.entity_disk_load= Some(parent.spawn((
                Mesh3d(meshes.add(block)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        unlit: true,
                        base_color: Color::srgb_u8(255, 50, 50),
                        ..default()
                    })
                ),
                Transform::from_translation(Vec3::new(0.1, -6.22, -0.2)),   
                Visibility::Hidden, 
            )).id());
            parent.spawn((
                Mesh3d(meshes.add(block)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        unlit: true,
                        base_color: back_color_load_block,
                        ..default()
                    })
                ),
                Transform::from_translation(Vec3::new(0.1, -6.22, -0.25)),   
            ));

            parent.spawn((
                Mesh3d(meshes.add(line_h)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(0.08, -5.72, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_h)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(0.08, -6.72, -0.12)),   
            ));

            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(2.23, -6.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(1.15, -6.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(0.05, -6.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-1.05, -6.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(-2.08, -6.22, -0.12)),   
            ));


            //RAM
            parent.spawn((
                TextMesh{
                    text: "Ram".to_string(),
                    font: asset_server
                        .load("fonts/Fira_Sans/FiraSans-ExtraLight.ttf"),
                    ..default()
                },
                MeshMaterial3d(materials.add(StandardMaterial {
                    unlit: true,
                    base_color: Color::srgb_u8(180, 190, 205),
                    ..default()
                })),
                Transform::from_xyz(3.9, -1.5, -0.3), // relativo al padre
            ));

            //Cargador
            server.entity_ram_load= Some(parent.spawn((
                Mesh3d(meshes.add(block)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        unlit: true,
                        base_color: Color::srgb_u8(255, 50, 50),
                        ..default()
                    })
                ),
                Transform::from_translation(Vec3::new(4.65, -3.22, -0.2)),   
                Visibility::Hidden, 
            )).id());
            parent.spawn((
                Mesh3d(meshes.add(block)),
                MeshMaterial3d(
                    materials.add(StandardMaterial {
                        unlit: true,
                        base_color: back_color_load_block,
                        ..default()
                    })
                ),
                Transform::from_translation(Vec3::new(4.65, -3.22, -0.25)),   
            ));

            parent.spawn((
                Mesh3d(meshes.add(line_h)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(4.65, -2.72, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_h)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(4.65, -3.72, -0.12)),   
            ));

            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(6.85, -3.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(5.75, -3.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(4.65, -3.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(3.55, -3.22, -0.12)),   
            ));
            parent.spawn((
                Mesh3d(meshes.add(line_v)),
                MeshMaterial3d(material_border.clone()),
                Transform::from_translation(Vec3::new(2.45, -3.22, -0.12)),   
            ));
        });


    }

}




pub fn get_metrics(
    server_list: Query<&visualizer::component::servers::Servers>,
    time: Res<Time>,
    mut metrics_timer: ResMut<visualizer::resource::server_metrics_run_timer::ServerMetricsRunTimer>,
    query_servers: Query<
        &visualizer::component::server::Server,
        (With<visualizer::component::server::ServerReadyMetrics>, Without<visualizer::component::server::ObtainingServerMetrics>)
    >,
)
{
    if !metrics_timer.timer.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(original_servers) = server_list.single() else {
        return;
    };

    for server_component in query_servers.iter() {
        
        let Some(core_server) = original_servers
            .list_original_servers
            .iter()
            .find(|server| server.uuid == server_component.uuid)
            .cloned()
        else {
            continue;
        };

        let tx= metrics_timer.tx.clone();
        core_server.async_get_metrics_avg(tx);

    }

}

pub fn get_metrics_first(
    server_list: Query<&visualizer::component::servers::Servers>,
    metrics_timer: ResMut<visualizer::resource::server_metrics_run_timer::ServerMetricsRunTimer>,
    query_servers: Query<
        &visualizer::component::server::Server,
        Added<visualizer::component::server::ServerReadyMetrics>
    >,
)
{
    let Ok(original_servers) = server_list.single() else {
        return;
    };

    for server_component in query_servers.iter() {
        
        let Some(core_server) = original_servers
            .list_original_servers
            .iter()
            .find(|server| server.uuid == server_component.uuid)
            .cloned()
        else {
            continue;
        };

        let tx= metrics_timer.tx.clone();
        core_server.async_get_metrics_avg(tx);

    }

}


pub fn receive_metrics(
    mut commands: Commands,
    metrics_timer: Res<visualizer::resource::server_metrics_run_timer::ServerMetricsRunTimer>,
    query_servers: Query< &visualizer::component::server::Server >,
)
{
    let metric= metrics_timer.rx.lock().unwrap().try_recv();
    if let Ok(metrics) = metric {
        match metrics {
            ServerMetrics::Done(server_uuid, cpu, ram, disk)=> {
                if let Some(server) = query_servers.iter().find(|server| server.uuid == Some(server_uuid)){
                    commands.entity(server.entity.unwrap()).insert(
                        visualizer::component::server::ServerMetrics{
                            cpu, ram, disk,
                        }
                    );
                }
            }
            ServerMetrics::Error(_uuid, error)=> {
                panic!("Error {}", error);
            }
        }
    }

}

pub fn update_metrics(
    mut commands: Commands,
    mut transforms: Query<&mut Transform>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_materials: Query<&MeshMaterial3d<StandardMaterial>>,
    query_servers: Query<
        (&visualizer::component::server::Server, &visualizer::component::server::ServerMetrics),
        Or<(Added<visualizer::component::server::ServerMetrics>, Changed<visualizer::component::server::ServerMetrics>)>
    >,
)
{
    let cube_width = 4.27;
    let mut change_color= |usage: f32, entity: Entity|{
        let color = match usage {
            0.0..=0.25 => Color::srgb_u8(47, 128, 237),   // Light
            0.26..=0.50 => Color::srgb_u8(24, 179, 104),  // Normal
            0.51..=0.75 => Color::srgb_u8(233, 162, 59),  // High
            _ => Color::srgb_u8(0xD9, 0x4A, 0x4A)     // Critical
        };
        if let Ok(mat_handle) = mesh_materials.get(entity) {
            if let Some(mut material) = materials.get_mut(mat_handle.clone()) {
                material.base_color = color;
            }
        }
    };
    for (server, metrics) in query_servers{
        if server.entity_cpu_load == None { continue; };
        if server.entity_ram_load == None { continue; };
        if server.entity_disk_load == None { continue; };
        if let Ok(mut transform) = transforms.get_mut(server.entity_cpu_load.unwrap()) {
            transform.scale.x = metrics.cpu;
            transform.translation.x = -4.65 - (cube_width * (1.0 - transform.scale.x)/2.0);
            change_color(metrics.cpu, server.entity_cpu_load.unwrap());

        }
        if let Ok(mut transform) = transforms.get_mut(server.entity_ram_load.unwrap()) {
            transform.scale.x = metrics.ram;
            transform.translation.x = 4.65 - (cube_width * (1.0 - transform.scale.x)/2.0);
            change_color(metrics.ram, server.entity_ram_load.unwrap());
        }
        if let Ok(mut transform) = transforms.get_mut(server.entity_disk_load.unwrap()) {
            transform.scale.x = metrics.disk;
            transform.translation.x = 0.1 - (cube_width * (1.0 - transform.scale.x)/2.0);
            change_color(metrics.disk, server.entity_disk_load.unwrap());
        }
        commands.entity(server.entity.unwrap()).remove::<visualizer::component::server::ServerMetrics>();
        commands.entity(server.entity_cpu_load.unwrap()).insert(Visibility::Inherited);
        commands.entity(server.entity_ram_load.unwrap()).insert(Visibility::Inherited);
        commands.entity(server.entity_disk_load.unwrap()).insert(Visibility::Inherited);
    }

}

