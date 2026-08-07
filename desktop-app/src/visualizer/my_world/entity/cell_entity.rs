use bevy::prelude::*;

use crate::visualizer::my_world;

pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: my_world::component::cell::Cell
)->Entity
{
    let hex_handle: Handle<WorldAsset> = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("gltf/hexagono.gltf"),
    );

    let entity= commands.spawn((
        WorldAssetRoot(hex_handle.clone()),
        Transform {
            translation: cell.position,
            scale: Vec3::splat(1.0),
            ..default()
        },
        cell
    )).id();

    entity

}