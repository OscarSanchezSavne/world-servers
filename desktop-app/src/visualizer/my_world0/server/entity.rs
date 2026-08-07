use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_fontmesh::TextMesh;

use crate::{core::server::manager, visualizer::my_world0::{self}};

pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: &my_world0::cell::components::Cell,
    server: &manager::Server,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    world_data: &mut my_world0::global::resources::WorldData
)->Entity
{
    // En pared:
    // X = izquierda/derecha
    // Y = arriba/abajo
    // Z = profundidad frente a la pared
    let server_handle: Handle<WorldAsset> = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("gltf/server.gltf"),
    );
    let server_entity= commands.spawn((
        WorldAssetRoot(server_handle.clone()),
        Transform {
            translation: cell.center,
            scale: Vec3::splat(1.0),
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),
            ..default()
        }
    )).with_children(|parent| {
        let text = server.server_name.to_string();
        let text_scale = 0.6;
        let char_width = 0.13;

        let text_width = text.len() as f32 * char_width;
        parent.spawn((
            TextMesh {
                text: text.to_string(),
                font: asset_server.load("fonts/Fira_Sans/FiraSans-ExtraLight.ttf"),
                ..default()
            },
            MeshMaterial3d(materials.add(StandardMaterial {
                unlit: true,
                alpha_mode: AlphaMode::Opaque,
                base_color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            })),
            Transform {
                translation: Vec3 {
                    x: -text_width,
                    y: 0.5,
                    z: -3.0,
                },
                rotation: Quat::from_rotation_x(-FRAC_PI_2),
                scale: Vec3::splat(text_scale),

                ..default()
            },
        ));
    }).id();

    world_data.server_entity_map.insert(server.uuid.unwrap(), server_entity);

    server_entity
    
}