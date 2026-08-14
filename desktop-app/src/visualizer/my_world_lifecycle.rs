use bevy::{DefaultPlugins, app::{App, Update}, ecs::system::Commands, state::app::AppExtStates};
use bevy_egui::EguiPrimaryContextPass;

use bevy::prelude::*;
use bevy_fontmesh::FontMeshPlugin;
use crate::visualizer;

pub fn big_bang() {
    App::new()
        .add_systems(Update, visualizer::system::camera_system::update_world_camera)
        .add_systems(Update, visualizer::system::camera_system::zoom)
        .add_systems(EguiPrimaryContextPass, visualizer::system::camera_system::camera_controls_help)
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(bevy::picking::mesh_picking::MeshPickingPlugin)
        .add_plugins(FontMeshPlugin::<StandardMaterial>::default())
        .insert_state(visualizer::data::world_data::WorldState::Start)

        .add_systems(Startup, (
            setup_resources,
            visualizer::entity::world_entity::create
        ))
        .add_systems(Update, visualizer::system::grid_system::build)
        
        .add_systems(Update, visualizer::system::server_system::create)
        .add_systems(EguiPrimaryContextPass, visualizer::system::log_system::update)
        .add_systems(EguiPrimaryContextPass, visualizer::system::panel_server_filters::update)

        .add_systems(Update, visualizer::system::server_system::connect)
        .add_systems(Update, visualizer::system::log_system::handle_server_ssh_messages)


        .add_systems(Update, visualizer::system::server_system::refresh_connection_state)
        .add_systems(Update, visualizer::system::grid_system::add_color_by_server_type)
        .add_systems(Update, visualizer::system::grid_system::pulse_cells)
        .add_systems(Update, visualizer::system::server_system::attach_cell_entity)
        .add_systems(Update, visualizer::system::server_system::capture_traffic)
        .add_systems(Update, visualizer::system::server_system::start_capture_metrics)
        .add_systems(Update, visualizer::system::server_system::get_metrics)
        .add_systems(Update, visualizer::system::server_system::receive_metrics)
        .add_systems(Update, visualizer::system::server_system::update_metrics)
        .add_systems(Update, visualizer::system::server_system::get_metrics_first)
        .add_systems(Update, visualizer::system::package_data_system::listen)
        .add_systems(Update, visualizer::system::package_data_system::spawn_packages)
        .add_systems(Update, visualizer::system::package_data_system::move_packages)
        
        .add_systems(Update, visualizer::system::toaster_system::show)
        .run();
    
}

fn setup_resources(
    mut commands: Commands,
) {
    commands.insert_resource(visualizer::resource::log_buffer::LogBuffer::default());
    commands.insert_resource(visualizer::resource::package_data_queue::PackageDataQueue::default());
    commands.insert_resource(visualizer::resource::server_filter_state::ServerFilterState::default());
    commands.insert_resource(visualizer::resource::camera::LastPosition::default());
    commands.insert_resource(visualizer::resource::toaster::Toaster::default());
    commands.spawn(visualizer::component::grid::Grid::create());
    commands.spawn(visualizer::component::servers::Servers::create());
    commands.insert_resource(visualizer::resource::server_metrics_run_timer::ServerMetricsRunTimer::default());
    
}
