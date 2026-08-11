use bevy::{prelude::*};

use crate::visualizer;

pub fn listen(
    mut package_data_queue: ResMut<visualizer::resource::package_data_queue::PackageDataQueue>
)
{
    loop {
        let package = package_data_queue.rx.lock().unwrap().try_recv();
        let Ok(package) = package else { break };
        package_data_queue.add(package);
    }
}


pub fn spawn_packages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    servers_query: Query< &visualizer::component::server::Server >,
    mut servers_list: Query<&mut visualizer::component::servers::Servers>,
    mut package_data_queue: ResMut<visualizer::resource::package_data_queue::PackageDataQueue>,
){
    let packages= package_data_queue.take();

    if packages.is_empty(){
        return;
    }

    let mut servers_list= servers_list.single_mut().unwrap();
    let mut not_found_ips:Vec<String>= Vec::new();

    for package in packages.into_iter(){
        let external_server_ip= if package.package_data.inbound{
            package.package_data.src_ip.clone()
        }else{
            package.package_data.dst_ip.clone()
        };
        if not_found_ips.contains(&external_server_ip){
            continue;
        }

        let server_internal= servers_query
            .iter()
            .find(|server|server.uuid.unwrap() == package.server_uuid)
            .unwrap();

        if !server_internal.visible{
            return;
        }

        let Some(server_external) = servers_query
            .iter()
            .find(|server| server.ip == external_server_ip)
        else {
            not_found_ips.push(external_server_ip.clone());
            servers_list.add_external_server(
                external_server_ip.clone(), external_server_ip.clone()
            );
            package_data_queue.add(package);
            continue;
        };

        if  server_external.entity == None || server_external.entity_cell == None{
            package_data_queue.add(package);
            continue;
        }

        if !server_external.visible {
            return;
        }

        let (server_source, server_target)= if package.package_data.inbound {
            (server_external, server_internal)
        }else{
            (server_internal, server_external)
        };

        visualizer::entity::package_data_entity::create(
            &mut commands, &mut meshes, &mut materials, server_source, package.package_data,
            visualizer::component::package_data::PackageData{
                target: Vec3::new(server_target.position.x, server_target.position.y, 2.5)
            }
        );

    }

}

pub fn move_packages(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &visualizer::component::package_data::PackageData)>,
) {
    let speed = 6.0; // unidades por segundo

    for (entity, mut transform, package_data) in &mut query {
        let to_target = package_data.target - transform.translation;
        let distance = to_target.length();

        // Llegó al destino → eliminar
        if distance <= 0.05 {
            commands.entity(entity).despawn();
            continue;
        }

        // Avanza sin sobrepasar el destino
        let step = speed * time.delta_secs();
        let movement = if step >= distance {
            to_target
        } else {
            to_target.normalize() * step
        };

        transform.translation += movement;
    }
}