use bevy::prelude::*;
use crate::{core::server::manager, visualizer::my_world0::{self, global::components::AppState}};

pub fn run(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_data: ResMut<my_world0::global::resources::WorldData>,
    mut next_state: ResMut<NextState<AppState>>
) {
    let servers= manager::Server::get_servers();

    let total_cells = servers.len() as i32 + 2;

    // En pared:
    // X = izquierda/derecha
    // Y = arriba/abajo
    // Z = profundidad frente a la pared
    let cell_z = 0.01;

    let mut cells = Vec::new();

    let cols = (total_cells as f32).sqrt().ceil() as i32;
    let rows = ((total_cells as f32) / cols as f32).ceil() as i32;

    let start_y = -(rows / 2);
    let start_x = -(cols / 2);

    for row in 0..rows {
        let y = start_y as f32 + ((world_data.cell_depth / 2.0) * row as f32);

        for col in 0..cols {
            let x = if row % 2 == 0 {
                start_x as f32 + (world_data.cell_width * 1.5 * col as f32)
            } else {
                (start_x as f32 + (world_data.cell_width * 1.5 * col as f32)) - 7.5
            };

            cells.push(my_world0::cell::components::Cell {
                center: Vec3::new(x, y, cell_z),
                size: 1.0,
                external: false,
            });
        }
    }


    for cell in cells.iter()
    {
        let entity= my_world0::cell::entity::create(
            &mut commands, &asset_server, cell.clone()
        );
        world_data.cells.push(entity);
    }

    let indices: Vec<usize> = (0..total_cells as usize).collect();
    world_data.index_cells_sort = my_world0::grid::utilities::reorder_from_center(&indices);
    next_state.set(AppState::GridLoaded);

}




