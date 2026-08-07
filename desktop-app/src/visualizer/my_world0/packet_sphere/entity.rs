use bevy::prelude::*;
use uuid::Uuid;

use crate::visualizer::my_world0::packet_sphere::{components::PacketSphere, resources::SphereAssets};

pub fn create(
    commands: &mut Commands,
    server_traffic_package: crate::ui::utilities::TcpdumpPacket,
    source_pos: Vec3,
    sphere_assets: &SphereAssets,
)
{
    let size= server_traffic_package.size as f32 / 25.0;

    let size:f32= if size<0.2{
        0.1
    } else if size > 1.0{
        0.5
    } else {
        size
    };
    let packet= PacketSphere {
        size: size,
        source: Vec3::new(source_pos.x, source_pos.y, 2.0),
        target: Vec3::new(50.0, 50.0, 2.0),
    };
    commands.spawn((
        Mesh3d(sphere_assets.mesh.clone()),
        MeshMaterial3d(sphere_assets.material.clone()),
        Transform {
            translation: packet.source,
            scale: Vec3::splat(packet.size),
            ..default()
        },
        packet,
    ));
}