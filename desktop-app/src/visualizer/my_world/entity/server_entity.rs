use bevy::prelude::*;

use crate::visualizer::my_world;



pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: my_world::component::cell::Cell,
    server: my_world::component::server::Server,
)->Entity
{
    let hex_handle: Handle<WorldAsset> = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("gltf/server.gltf"),
    );

    let server_entity= commands.spawn((
        WorldAssetRoot(hex_handle.clone()),
        Transform {
            translation: Vec3 { x: cell.position.x + 6.3, y: cell.position.y - 6.8, z: 0.25},
            scale: Vec3::splat(0.8),
            ..default()
        },
        server.clone(),
        cell.clone(),
    )).id();

    commands.spawn((
        Text::new(server.name),
        TextFont {
            font: asset_server
                .load("fonts/Fira_Sans/FiraSans-ExtraLight.ttf")
                .into(),
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::WHITE),

        Node {
            position_type: PositionType::Absolute,
            ..default()
        },

        my_world::component::server::Label {
            server: server_entity,
        },
    ));
    server_entity

}
