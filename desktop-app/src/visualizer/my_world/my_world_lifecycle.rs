use bevy::{DefaultPlugins, app::{App, Update}, camera_controller::free_camera::FreeCameraPlugin, ecs::{resource::Resource, system::{Commands, Query, ResMut, Single}}, state::{app::AppExtStates}, time::{Time, Timer, TimerMode}, window::Window};
use bevy_egui::EguiPrimaryContextPass;

use crate::visualizer::my_world;

pub fn big_bang() {
    App::new()
        .add_plugins(FreeCameraPlugin)
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin::default())
        .insert_state(my_world::data::world_data::WorldState::Start)

        .add_systems(Startup, (
            setup_resources,
            my_world::entity::world_entity::create
        ))
        .add_systems(Update, my_world::system::grid_system::build)
        .add_systems(Update, my_world::system::server_system::create)
        .add_systems(EguiPrimaryContextPass, my_world::system::log_system::update)
        //.add_systems(Update, testing)
        .add_systems(
            Update, my_world::system::server_label_system::update_positions.run_if(
                in_state(my_world::data::world_data::WorldState::GridLoaded)
            )
        )
        .add_systems(Update, my_world::system::server_system::connect)
        .add_systems(Update, my_world::system::log_system::handle_server_ssh_messages)
        .add_systems(Update, my_world::system::server_system::refresh_connection_state)
        .add_systems(Update, my_world::system::grid_system::add_color_by_server_type)
        .add_systems(Update, my_world::system::grid_system::pulse_cells)
        .add_systems(Update, my_world::system::server_system::attach_cell_entity)
        .run();
    
}

fn setup_resources(
    mut commands: Commands,
) {
    commands.insert_resource(
        GridExpandTimer(Timer::from_seconds(100.0, TimerMode::Repeating))
    );

    commands.insert_resource(my_world::resource::log_buffer::LogBuffer::default());
    commands.spawn(my_world::component::grid::Grid::create());
    commands.spawn(my_world::component::servers::Servers::create());
}

fn maximize_window(mut _window: Single<&mut Window>) {
    //window.set_maximized(true);
}


use bevy::prelude::*;

#[derive(Resource)]
pub struct GridExpandTimer(pub Timer);



pub fn testing(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut gizmos: Gizmos,
    time: Res<Time>,
    mut timer: ResMut<GridExpandTimer>,
    mut grid_query: Query<&mut my_world::component::grid::Grid>
) 
{
    /*gizmos.text_2d(
        Isometry2d::from_translation(Vec2::new(0.0, 0.0)),
        "ACA ESTOOOOY---",
        2.0,
        Vec2::ZERO,
        Color::WHITE,
    );*/
    gizmos.line(Vec3::new(0.0, 0.0, 0.5), Vec3::new(12.5, 0.0, 0.5), Color::srgb(1.0, 0.0, 0.0),);
    gizmos.line(Vec3::new(0.0, 0.0, 0.5), Vec3::new(0.0, -13.5, 0.5), Color::srgb(1.0, 0.0, 0.0),);
    if let Ok(mut camera) = camera_query.single_mut() {
        //println!("Camera x:{} y:{} z:{}", camera.translation.x, camera.translation.y,camera.translation.z);
        // Centrar cámara en el nuevo grid
        /*let center_y = (grid.rows as f32 * 18.0) / 1.0;
        camera.translation.y = center_y; // mueve la cámara hacia arriba
        let center_x = (grid.cols as f32 * 7.5) / 1.0;
        camera.translation.y = center_x; // mueve la cámara hacia arriba*/
    }

    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    if let Ok(mut grid) = grid_query.single_mut() {
        grid.expand_border();
    }

} 