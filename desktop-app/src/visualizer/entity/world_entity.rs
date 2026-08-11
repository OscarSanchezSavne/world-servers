use bevy::{camera::Exposure, core_pipeline::tonemapping::Tonemapping, post_process::bloom::Bloom, prelude::*, render::view::{ColorGrading, ColorGradingGlobal}};

use crate::visualizer;

pub fn create(
    mut commands: Commands,
) {
    // Fondo de la escena
    commands.insert_resource(ClearColor(Color::srgb_u8(0x08, 0x11, 0x1D)));

    // Cámara
    let focus = Vec3::new(24.98, -11.34, 0.0);
    let distance = 41.12;

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(focus.x, focus.y, focus.z + distance)
        .looking_at(focus, Vec3::Y),
        visualizer::component::world_camera::WorldCamera::new(focus, distance),
        Tonemapping::KhronosPbrNeutral,
        ColorGrading {
            global: ColorGradingGlobal { post_saturation: 1.05, ..default() },
            ..default()
        },
        Exposure { ev100: 9.5 },
        Bloom { intensity: 0.06, ..Bloom::NATURAL },
    ));


    // Luz ambiental
    commands.insert_resource(
        GlobalAmbientLight {
            color: Color::srgb_u8(0xD0, 0xDC, 0xEE),
            brightness: 180.0,
            ..default()
        }
    );

    // "Sol"
    commands.spawn((
        DirectionalLight {
            illuminance: 4500.0,
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, -0.20, -0.15, 0.0)
        ),
    ));

}
