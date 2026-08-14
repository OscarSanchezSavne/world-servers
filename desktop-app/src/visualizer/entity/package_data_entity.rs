use bevy::{light::NotShadowCaster, prelude::*};

use crate::{ui::utilities::TcpdumpPacket, visualizer};

        
const MIN_SIZE: f32 = 64.0;   // paquete TCP mínimo típico
const MAX_SIZE: f32 = 1514.0; // MTU Ethernet típico

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
        .clamp(0.1, 0.6);

    package_data.target.x += 0.5;

    let alpha= if normalized > 0.4 { 0.4 } 
    else if normalized > 0.2 { 0.6 } 
    else { 1.0 };


    let (color, emissive) = if package.internal.unwrap_or(false) {
        if package.inbound {
            (
                // Interno entrante - Violeta
                Color::srgba(0.55, 0.31, 0.96, alpha),
                LinearRgba::rgb(0.20, 0.04, 0.45),
            )
        } else {
            (
                // Interno saliente - Magenta
                Color::srgba(0.93, 0.22, 0.55, alpha),
                LinearRgba::rgb(0.40, 0.03, 0.16),
            )
        }
    } else if package.inbound {
        (
            // Externo entrante - Azul
            Color::srgba(0.18, 0.50, 0.93, alpha),
            LinearRgba::rgb(0.02, 0.12, 0.45),
        )
    } else {
        (
            // Externo saliente - Naranja
            Color::srgba(0.91, 0.48, 0.09, alpha),
            LinearRgba::rgb(0.45, 0.12, 0.01),
        )
    };

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(normalized))),
        MeshMaterial3d(
            materials.add(
                StandardMaterial {
                    base_color: color,
                    emissive: emissive,
                    alpha_mode: AlphaMode::Blend,
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
            ..default()
        },

        package_data,
    ))
    .id()
}
