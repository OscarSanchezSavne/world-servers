use bevy::{
    input::mouse::{
        AccumulatedMouseMotion,
        AccumulatedMouseScroll,
        MouseScrollUnit,
    },
    prelude::*,
};

use crate::visualizer;

pub fn update_world_camera(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<
        (&mut Transform, &mut visualizer::component::world_camera::WorldCamera),
        With<Camera3d>,
    >,
) {
    let Ok((mut transform, mut camera)) = camera_query.single_mut() else {
        return;
    };

    let delta = mouse_motion.delta;

    // -----------------------------------------------------
    // BOTÓN Derecho
    // Rotar/orbitar alrededor del tablero
    // -----------------------------------------------------

    if mouse_buttons.pressed(MouseButton::Right) {
        camera.yaw -= delta.x * camera.rotate_sensitivity;
        camera.pitch -= delta.y * camera.rotate_sensitivity;

        // Evitar voltear completamente la cámara
        camera.pitch = camera.pitch.clamp(-1.45, 1.45);
    }

    // -----------------------------------------------------
    // BOTÓN IZQUIERDO
    // Arrastrar / pan
    // -----------------------------------------------------

    if mouse_buttons.pressed(MouseButton::Left) {
        let right = transform.rotation * Vec3::X;
        let up = transform.rotation * Vec3::Y;

        // Pan proporcional a la distancia.
        let scale = camera.distance * camera.pan_sensitivity;

        camera.focus += (-right * delta.x + up * delta.y) * scale;
    }

    // -----------------------------------------------------
    // RUEDA
    // Zoom
    // -----------------------------------------------------

     let mut zoom = match mouse_scroll.unit {
        MouseScrollUnit::Line => mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => mouse_scroll.delta.y * 0.02,
    };

    // + normal
    if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
        zoom += 0.2;
    }

    // - normal
    if keyboard_input.pressed(KeyCode::Minus) || keyboard_input.pressed(KeyCode::NumpadSubtract) {
        zoom -= 0.2;
    }

    if zoom != 0.0 {
        camera.distance *=
            (-zoom * camera.zoom_sensitivity).exp();

        camera.distance = camera.distance.clamp(
            camera.min_distance,
            camera.max_distance,
        );
    }
    
    // -----------------------------------------------------
    // CALCULAR POSICIÓN
    // -----------------------------------------------------

    let cos_pitch = camera.pitch.cos();

    let offset = Vec3::new(
        camera.distance * cos_pitch * camera.yaw.sin(),
        camera.distance * camera.pitch.sin(),
        camera.distance * cos_pitch * camera.yaw.cos(),
    );

    transform.translation = camera.focus + offset;

    transform.look_at(camera.focus, Vec3::Y);

}