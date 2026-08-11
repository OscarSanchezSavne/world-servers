use bevy::{DefaultPlugins, app::{App, Update}, ecs::system::Commands, state::app::AppExtStates};
use bevy_egui::EguiPrimaryContextPass;

use bevy::prelude::*;
use crate::visualizer;

pub fn big_bang() {
    App::new()
        .add_systems(Update, visualizer::system::camera_system::update_world_camera)
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin::default())
        .insert_state(visualizer::data::world_data::WorldState::Start)

        .add_systems(Startup, (
            setup_resources,
            visualizer::entity::world_entity::create
        ))
        .add_systems(Update, visualizer::system::grid_system::build)
        
        .add_systems(Update, visualizer::system::server_system::create)
        .add_systems(EguiPrimaryContextPass, visualizer::system::log_system::update)
        .add_systems(EguiPrimaryContextPass, visualizer::system::panel_server_filters::update)

        .add_systems(
            Update, visualizer::system::server_label_system::update_positions.run_if(
                in_state(visualizer::data::world_data::WorldState::GridLoaded)
            )
        )
        .add_systems(Update, visualizer::system::server_system::connect)
        .add_systems(Update, visualizer::system::log_system::handle_server_ssh_messages)


        .add_systems(Update, visualizer::system::server_system::refresh_connection_state)
        .add_systems(Update, visualizer::system::grid_system::add_color_by_server_type)
        .add_systems(Update, visualizer::system::grid_system::pulse_cells)
        .add_systems(Update, visualizer::system::server_system::attach_cell_entity)
        .add_systems(Update, visualizer::system::server_system::capture_traffic)
        .add_systems(Update, visualizer::system::package_data_system::listen)
        .add_systems(Update, visualizer::system::package_data_system::spawn_packages)
        .add_systems(Update, visualizer::system::package_data_system::move_packages)
        .run();
    
}

fn setup_resources(
    mut commands: Commands,
) {
    commands.insert_resource(visualizer::resource::log_buffer::LogBuffer::default());
    commands.insert_resource(visualizer::resource::package_data_queue::PackageDataQueue::default());
    commands.insert_resource(visualizer::resource::server_filter_state::ServerFilterState::default());
    commands.spawn(visualizer::component::grid::Grid::create());
    commands.spawn(visualizer::component::servers::Servers::create());
}


