use bevy::prelude::*;

use crate::visualizer;



pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: visualizer::component::cell::Cell,
    server: &mut visualizer::component::server::Server,
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
    )).id();
    
    new_server_component.entity= Some(server_entity);

    commands.entity(server_entity).insert(new_server_component);

    commands.spawn((
        Text::new(server.name.clone()),
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

        visualizer::component::server::Label {
            server: server_entity,
        },
    ));
    server_entity

}
