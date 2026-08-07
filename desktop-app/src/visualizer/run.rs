use bevy::{camera_controller::free_camera::{FreeCameraPlugin}, prelude::*};
use bevy_fontmesh::prelude::*;

use crate::visualizer::my_world0::{self, global::{components::AppState}};


pub fn world_3d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(AppState::Start)
        .insert_resource(my_world0::server::resources::TrafficChannel::default())
        .insert_resource(my_world0::log::resource::LogBuffer::default())

        .add_systems(Startup, my_world0::cell::resources::setup_materials)
        .add_systems(Startup, my_world0::packet_sphere::resources::setup_materials)

        .add_plugins(FreeCameraPlugin)
        .add_plugins(FontMeshPlugin::<StandardMaterial>::default())
        .add_observer(my_world0::global::systems::on_model_ready::observe)
        .add_systems(Startup, maximize_window)

        .add_systems(OnEnter(AppState::Start), my_world0::global::systems::setup::run)
        .add_systems(OnEnter(AppState::GlobalLoaded), my_world0::grid::systems::init::run)
        .add_systems(OnEnter(AppState::GridLoaded), my_world0::server::systems::init::run)

        .add_systems(Update, my_world0::server::systems::traffic::capture)
        .add_systems(Update, my_world0::cell::systems::cell_state::update)
        .add_systems(Update, my_world0::server::systems::listener::server_ready)
        .add_systems(Update, my_world0::server::systems::listener::server_traffic)
        .add_systems(Update, my_world0::packet_sphere::systems::move_traffic::run)

        .run();
    
}


fn maximize_window(mut _window: Single<&mut Window>) {
    //window.set_maximized(true);
}

