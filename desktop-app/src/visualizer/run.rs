use bevy::{camera_controller::free_camera::{FreeCameraPlugin}, prelude::*};
use bevy_egui::EguiPrimaryContextPass;
use bevy_fontmesh::prelude::*;

use crate::visualizer::my_world::{self, global::{components::AppState}};


pub fn world_3d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_state(AppState::Start)
        .insert_resource(my_world::log::resource::LogBuffer::default())
        .add_plugins(FreeCameraPlugin)
        .add_plugins(bevy_egui::EguiPlugin {
            ui_render_order: bevy_egui::UiRenderOrder::EguiAboveBevyUi, // número alto = se dibuja al final, encima de todo
            ..Default::default()
        })
        .add_plugins(FontMeshPlugin::<StandardMaterial>::default())
        .add_observer(my_world::global::systems::on_model_ready::observe)
        .add_systems(Startup, maximize_window)
        .add_systems(Startup, my_world::cell::resources::setup_materials)

        .add_systems(OnEnter(AppState::Start), my_world::global::systems::setup::run)
        .add_systems(OnEnter(AppState::GlobalLoaded), my_world::grid::systems::init::run)
        .add_systems(OnEnter(AppState::GridLoaded), my_world::server::systems::init::run)

        .add_systems(Update, my_world::cell::systems::cell_state::update)
        .add_systems(Update, my_world::server::systems::listener::server_ready)
        .add_systems(EguiPrimaryContextPass, my_world::log::systems::setup::run.run_if(in_state(AppState::GridLoaded)))


        .run();
}


fn maximize_window(mut _window: Single<&mut Window>) {
    //window.set_maximized(true);
}

