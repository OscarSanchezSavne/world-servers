use std::f32::consts::TAU;

use bevy::ecs::{query::{Added, Changed, Or}, system::Query};
use bevy::prelude::*;

use crate::visualizer;

pub fn build(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_state: ResMut<NextState<visualizer::data::world_data::WorldState>>,
    current_world_state: Res<State<visualizer::data::world_data::WorldState>>,
    mut query: Query<
        &mut visualizer::component::grid::Grid, 
        Or<(Added<visualizer::component::grid::Grid>, Changed<visualizer::component::grid::Grid>)>
    >,
) 
{
    if let Ok(mut grid) = query.single_mut() {
        for row in grid.matrix_cells.iter_mut() {
            for cell in row.iter_mut() {
                if cell.entity != None {
                    continue;
                }
                cell.entity= Some(visualizer::entity::cell_entity::create(
                    &mut commands, &asset_server,
                    cell.clone()
                ));
            }
        }
        if *current_world_state.get() == visualizer::data::world_data::WorldState::Start{
            world_state.set(visualizer::data::world_data::WorldState::GridLoaded);
        }
    }
}



pub fn add_color_by_server_type(
    mut commands: Commands,
    children: Query<&Children>,
    mesh_query: Query<Entity, With<MeshMaterial3d<StandardMaterial>>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    query_servers: Query<
        (Entity, &visualizer::component::cell::Cell, &visualizer::component::cell::CellType),
        Added<visualizer::component::cell::CellType>,
    >,
    name_query: Query<&Name>,
) {
    for (cell_entity, _cell, server_type) in &query_servers {

        let color = match server_type {
            visualizer::component::cell::CellType::Internal => Color::srgb_u8(0x12, 0x34, 0x38),
            visualizer::component::cell::CellType::External => Color::srgb_u8(0x3A, 0x2B, 0x20),
        };

        let material = materials.add(StandardMaterial {
            base_color: color,
            emissive: LinearRgba::BLACK,
            metallic: 0.0,
            perceptual_roughness: 0.78,
            ..default()
        });

        for descendant in children.iter_descendants(cell_entity) {
            let Ok(name) = name_query.get(descendant) else {
                continue;
            };

            if !name.as_str().starts_with("internal_floor") {
                continue;
            }

            for child in children.iter_descendants(descendant) {
                if !mesh_query.contains(child) {
                    continue;
                }

                commands
                    .entity(child)
                    .insert(MeshMaterial3d(material.clone()));
            }
        }
    }
}


pub fn pulse_cells(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(
        Entity,
        &visualizer::component::cell::Cell,
        &mut visualizer::component::cell::PulsingCell,
        &mut Transform,
    )>,
) {
    // Un ciclo completo cada 2.4 segundos.
    const PERIOD: f32 = 1.4;

    // Solo reduce un 3.5 %
    const MIN_SCALE: f32 = 0.8;

    // Centro visual de tu celda.
    const CENTER_OFFSET_X: f32 = 6.3;
    const CENTER_OFFSET_Y: f32 = -6.8;

    let dt = time.delta_secs();

    for (entity, cell, mut pulse, mut transform) in &mut query {
        pulse.elapsed += dt;

        let phase = (pulse.elapsed / PERIOD) * TAU;

        // Empieza completamente quieto, se contrae suavemente y vuelve.
        let wave = (1.0 - phase.cos()) * 0.5;

        let scale = 1.0 - (1.0 - MIN_SCALE) * wave;

        let pivot = Vec3::new(
            cell.position.x + CENTER_OFFSET_X,
            cell.position.y + CENTER_OFFSET_Y,
            cell.position.z,
        );

        // Mantener el centro fijo mientras cambia la escala.
        let original_offset = cell.position - pivot;

        transform.translation =
            pivot + original_offset * scale;

        transform.scale = Vec3::splat(scale);

        // Si se pidió detener, esperamos a terminar suavemente el ciclo actual.
        if pulse.stop {
            let normalized_phase =
                pulse.elapsed.rem_euclid(PERIOD) / PERIOD;

            // Estamos nuevamente casi en escala 1.0.
            if normalized_phase < 0.02 {
                transform.translation = cell.position;
                transform.scale = Vec3::ONE;

                commands
                    .entity(entity)
                    .remove::<visualizer::component::cell::PulsingCell>();
            }
        }
    }
}


