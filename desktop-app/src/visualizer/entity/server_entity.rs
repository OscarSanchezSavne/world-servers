use bevy::{prelude::*, sprite::Text2dShadow};
use bevy_egui::EguiContexts;
use bevy_fontmesh::TextMesh;

use crate::visualizer;



pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: visualizer::component::cell::Cell,
    server: &mut visualizer::component::server::Server,
    materials: &mut Assets<StandardMaterial>,
)->Entity
{
    let hex_handle: Handle<WorldAsset> = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("gltf/server.gltf"),
    );

    server.position= Vec3 { x: cell.position.x + 6.3, y: cell.position.y - 6.8, z: 0.25};

    let mut new_server_component= server.clone();
    let server_entity= commands.spawn((
        WorldAssetRoot(hex_handle.clone()),
        Transform {
            translation: new_server_component.position,
            scale: Vec3::splat(0.8),
            ..default()
        },
        cell.clone(),
    )).observe(click_handler).id();
    
    new_server_component.entity= Some(server_entity);

    commands.entity(server_entity).insert(new_server_component);

    commands.entity(server_entity).with_children(|parent| {
        let margin= server.name.len() as f32 * 0.14;
        parent.spawn((
            TextMesh{
                text: server.name.clone(),
                font: asset_server
                    .load("fonts/Fira_Sans/FiraSans-ExtraLight.ttf"),
                ..default()
            },
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb_u8(0xE2, 0xE9, 0xF3),
                unlit: true,
                ..default()
            })),
            Text2dShadow {
                offset: Vec2::new(0.5, -0.5),
                color: Color::srgba(0.0, 0.0, 0.0, 1.0),
            },
            Transform {
                translation: Vec3::new(0.0 - margin, 3.8, 0.35),
                scale: Vec3::splat(0.7),
                ..default()
            },
        ));
    });

    server_entity

}

fn click_handler(
    click: On<Pointer<Click>>,
    servers: Query<&visualizer::component::server::Server>,
    mut contexts: EguiContexts,
    mut toaster: ResMut<visualizer::resource::toaster::Toaster>,
    mut commands: Commands,
    mut camera_query: Query<
        (Entity, &mut visualizer::component::world_camera::WorldCamera),
        With<Camera3d>,
    >,
) {
    if click.count >= 2 {

        let Ok((entity_camera, camera)) = camera_query.single_mut() else {
            return;
        };

        let Ok(server) = servers.get(click.entity) else {
            return;
        };

        commands.entity(entity_camera).insert(
            visualizer::component::world_camera::Zoom{
                target: server.position,
                target_distance: 25.0,
                start_focus: camera.focus,
                start_distance: camera.distance,
                elapsed: 0.0,
            }
        );
        return;
    }
    
    if let Ok(server) = servers.get(click.entity) {
        let Ok(ctx) = contexts.ctx_mut() else { return; };
        ctx.copy_text(format!("Name: {}\nIp: {}", server.name, server.ip));
        toaster.add("Copied".to_string());
    }
}