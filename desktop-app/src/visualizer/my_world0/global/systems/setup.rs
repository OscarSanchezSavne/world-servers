use std::f32::consts::FRAC_PI_2;

use bevy::{camera_controller::free_camera::{FreeCamera}, prelude::*};

use crate::visualizer::my_world0::{self, global::components::AppState};

pub fn run(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut next_state: ResMut<NextState<AppState>>,
) {

    commands.insert_resource(my_world0::global::resources::WorldData::default());

    // Cámara mirando a una pared en X/Y
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 100.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        FreeCamera::default(),
    ));

    // Pared: plano gris oscuro grande en X/Y
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.05, 0.05, 0.08),
            ..default()
        })),
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::from_rotation_x(FRAC_PI_2),
            ..default()
        },
    ));

    // Luz direccional hacia la pared
    commands.spawn((
        DirectionalLight {
            illuminance: 8000.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4)
        ),
    ));

    // Luz puntual frente a la pared
    commands.spawn((
        PointLight {
            intensity: 300.0,
            ..default()
        },
        Transform::from_xyz(0.0, 20.0, 40.0),
    ));

    next_state.set(AppState::GlobalLoaded);

}
