use bevy::prelude::*;

use crate::visualizer::my_world0::{self, global::components::{ModelNodes, ModelReady}};

const EXTERNAL_FLOORS: [&str; 6] = [
    "external_floor_01",
    "external_floor_02",
    "external_floor_03",
    "external_floor_04",
    "external_floor_05",
    "external_floor_06",
];

pub fn update(
    query: Query<
        (&my_world0::cell::components::CellState, &ModelNodes),
        (
            With<my_world0::cell::components::Cell>,
            With<ModelReady>,
            Or<(
                Changed<my_world0::cell::components::CellState>,
                Added<ModelNodes>,
            )>,
        ),
    >,
    hexagon_materials: Res<my_world0::cell::resources::CellMaterials>,
    mut material_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
) {
    for (state, nodes) in &query {
        let material = match state {
            my_world0::cell::components::CellState::Unassigned => &hexagon_materials.unassigned,
            my_world0::cell::components::CellState::Assigned => &hexagon_materials.assigned,
            my_world0::cell::components::CellState::Processing => &hexagon_materials.processing,
            my_world0::cell::components::CellState::Failed => &hexagon_materials.failed,
            my_world0::cell::components::CellState::InLine => &hexagon_materials.inline,
        };

        for floor_name in EXTERNAL_FLOORS {
            let Some(mesh_entity) = nodes.by_name.get(floor_name) else {
                println!("No existe nodo: {floor_name}");
                continue;
            };

            let Ok(mut mesh_material) = material_query.get_mut(*mesh_entity) else {
                println!("No se pudo obtener material de {floor_name}");
                continue;
            };

            mesh_material.0 = material.clone();
        }
    }
}