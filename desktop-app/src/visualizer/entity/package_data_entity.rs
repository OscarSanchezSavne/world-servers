use bevy::{light::NotShadowCaster, prelude::*};

use crate::{ui::utilities::TcpdumpPacket, visualizer};

        
const MIN_SIZE: f32 = 64.0;   // paquete TCP mínimo típico
const MAX_SIZE: f32 = 1514.0; // MTU Ethernet típico
const MIN_RADIUS: f32 = 0.05;
const MAX_RADIUS: f32 = 1.0;

pub fn create(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    server_source: &visualizer::component::server::Server,
    package: TcpdumpPacket,
    mut package_data: visualizer::component::package_data::PackageData,
) -> Entity {
    let size = package.size as f32;
    let normalized = ((size.ln() - MIN_SIZE.ln()) / (MAX_SIZE.ln() - MIN_SIZE.ln()))
        .clamp(0.0, 1.0);

    let size = MIN_RADIUS + normalized * (MAX_RADIUS - MIN_RADIUS);

    let direction = Vec2::new(
        package_data.target.x - server_source.position.x,
        package_data.target.y - server_source.position.y,
    );

    let angle = if direction.length_squared() > f32::EPSILON {
        direction.y.atan2(direction.x)
    } else { 0.0 };

    package_data.target.x= package_data.target.x+0.5;

    let (color, emissive) = if package.inbound {(
            // Azul
            Color::srgb_u8(0x2F, 0x80, 0xED),
            LinearRgba::rgb(0.02, 0.12, 0.45),
    )} else {(
            // Naranja
            Color::srgb_u8(0xE9, 0x7B, 0x18),
            LinearRgba::rgb(0.45, 0.12, 0.01),
    )};

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(size * 0.75, size * 0.40, size * 0.25))),
        MeshMaterial3d(
            materials.add(
                StandardMaterial {
                    base_color: color,
                    emissive: emissive,
                    metallic: 0.0,
                    reflectance: 0.0,
                    perceptual_roughness: 0.85,
                    ..default()
                }
            )
        ),
        NotShadowCaster,
        Transform {
            translation: Vec3::new(
                server_source.position.x,
                server_source.position.y,
                2.5,
            ),
            rotation: Quat::from_rotation_z(angle),
            ..default()
        },

        package_data,
    ))
    .id()
}
