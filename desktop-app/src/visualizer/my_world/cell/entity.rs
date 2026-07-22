use bevy::prelude::*;

use crate::visualizer::my_world::{self, cell::components::CellState};

pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: my_world::cell::components::Cell
)->Entity
{
    let hex_handle: Handle<WorldAsset> = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("gltf/hexagono.gltf"),
    );

    commands.spawn((
        WorldAssetRoot(hex_handle.clone()),
        Transform {
            translation: cell.center,
            scale: Vec3::splat(cell.size),
            // Rota el hexágono para que quede en la pared X/Y
            rotation: Quat::from_rotation_x(std::f32::consts::FRAC_PI_2),

            ..default()
        },
        cell.clone(),
        CellState::Unassigned
    )).id()
}