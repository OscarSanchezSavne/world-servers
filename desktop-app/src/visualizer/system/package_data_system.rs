use bevy::{prelude::*};

use crate::visualizer;

pub fn listen(
    mut package_data_queue: ResMut<visualizer::resource::package_data_queue::PackageDataQueue>
)
{
    const MAX_PACKAGES_PER_FRAME: usize = 200;
    let mut received = Vec::new();
    {
        let rx = package_data_queue.rx.lock().unwrap();
        for _ in 0..MAX_PACKAGES_PER_FRAME {
            let Ok(package) = rx.try_recv() else {
                break;
            };

            received.push(package);
        }
    }

    for package in received {
        package_data_queue.add(package);
    }
}


pub fn spawn_packages(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    servers_query: Query<&visualizer::component::server::Server >,
    mut servers_list: Query<&mut visualizer::component::servers::Servers>,
    mut package_data_queue: ResMut<visualizer::resource::package_data_queue::PackageDataQueue>,
){
    let packages= package_data_queue.take();

    if packages.is_empty(){
        return;
    }
    let mut servers_list= servers_list.single_mut().unwrap();
    let mut not_found_ips:Vec<String>= Vec::new();

    for mut package in packages.into_iter(){
        let peer_ip = if package.package_data.inbound {
            package.package_data.src_ip.clone()
        } else {
            package.package_data.dst_ip.clone()
        };

        if not_found_ips.contains(&peer_ip) {
            package_data_queue.add(package);
            continue;
        }

        let Some(server_internal) = servers_query
            .iter()
            .find(|server| {
                server.uuid == Some(package.server_uuid)
            })
        else {
            continue;
        };

        if !server_internal.visible { continue; }

        let Some(server_peer) = servers_query
            .iter()
            .find(|server| server.ip == peer_ip)
        else {
            not_found_ips.push(peer_ip.clone());

            servers_list.add_external_server(
                peer_ip.clone(),
                peer_ip,
            );

            package_data_queue.add(package);

            continue;
        };

        if server_peer.entity.is_none() || server_peer.entity_cell.is_none() {
            package_data_queue.add(package);
            continue;
        }

        if !server_peer.visible { continue; }

        let (server_source, server_target)= if package.package_data.inbound {
            (server_peer, server_internal)
        } else {
            (server_internal, server_peer)
        };

        if package.package_data.internal == Some(false) && !server_source.external && !server_target.external{
            package.package_data.internal= Some(true);
        }
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
    //println!("Packages {}", query.count());
    let base_speed = 6.0;        
    let distance_factor = 0.8;   

    for (entity, mut transform, package_data) in &mut query {
        let to_target = package_data.target - transform.translation;
        let distance = to_target.length();

        if distance <= 0.05 {
            commands.entity(entity).despawn();
            continue;
        }

        // A mayor distancia, mayor velocidad
        let speed = base_speed + distance_factor * distance;
        let step = speed * time.delta_secs();
        let movement = if step >= distance {
            to_target
        } else {
            to_target.normalize() * step
        };

        transform.translation += movement;
    }
}
