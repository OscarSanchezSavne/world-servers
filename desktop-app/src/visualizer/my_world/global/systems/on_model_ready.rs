use std::collections::HashMap;

use bevy::{prelude::*, world_serialization::WorldInstanceReady};

use crate::visualizer::my_world;

//función se ejecuta cada vez que una instancia de WorldAssetRoot queda lista
//Procesa eventos por lo cual no se puede por query
pub fn observe(
    scene_ready: On<WorldInstanceReady>,
    mut commands: Commands,
    cell_query: Query<(), With<my_world::cell::components::Cell>>,
    children_query: Query<&Children>,
    child_of_query: Query<&ChildOf>,
    name_query: Query<&Name>,
    material_query: Query<&MeshMaterial3d<StandardMaterial>>,
) {
    let cell_entity = scene_ready.entity;

    if !cell_query.contains(cell_entity) {
        return;
    }

    let mut by_name = HashMap::new();

    for descendant in children_query.iter_descendants(cell_entity) {
        if material_query.get(descendant).is_err() {
            continue;
        }

        let Ok(child_of) = child_of_query.get(descendant) else {
            continue;
        };

        let parent_entity = child_of.parent();

        let Ok(parent_name) = name_query.get(parent_entity) else {
            continue;
        };

        by_name.insert(parent_name.as_str().to_owned(), descendant);
    }

    commands.entity(cell_entity).insert((
        my_world::global::components::ModelReady,
        my_world::global::components::ModelNodes { by_name },
    ));
}