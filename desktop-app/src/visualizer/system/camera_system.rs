use bevy::{
    input::mouse::{
        AccumulatedMouseMotion,
        AccumulatedMouseScroll,
        MouseScrollUnit,
    },
    prelude::*,
};
use bevy_egui::{EguiContexts, egui};

use crate::visualizer;

pub fn update_world_camera(
    mut contexts: EguiContexts,
    mut commands: Commands,
    camera_last_position: ResMut<visualizer::resource::camera::LastPosition>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<
        (Entity, &mut Transform, &mut visualizer::component::world_camera::WorldCamera),
        (With<Camera3d>, Without<visualizer::component::world_camera::Zoom>),
    >,
) {
    let Ok((entity_camera, mut transform, mut camera)) = camera_query.single_mut() else {
        return;
    };

    // Si el puntero está sobre cualquier área egui, no tocar la cámara
    let Ok(ctx) = contexts.ctx_mut() else { return; };
    if ctx.is_pointer_over_egui() {
        return;
    }

    let delta = mouse_motion.delta;

    // Reestablecer camara
    if keyboard_input.pressed(KeyCode::Numpad0) {
        camera.yaw = 0.0;
        camera.pitch = 0.0;
        return;
    }

    // Reestablecer posicion anterior a zoom
    if keyboard_input.pressed(KeyCode::Escape) && camera_last_position.position != Vec3::ZERO{

        commands.entity(entity_camera).insert(
            visualizer::component::world_camera::Zoom{
                target: camera_last_position.position,
                target_distance: camera_last_position.distance,
                start_focus: camera.focus,
                start_distance: camera.distance,
                elapsed: 0.0,
            }
        );
        return;
    }

    // BOTÓN DERECHO Girar
    if mouse_buttons.pressed(MouseButton::Right) {
        camera.yaw -= delta.x * camera.rotate_sensitivity;
        camera.pitch -= delta.y * camera.rotate_sensitivity;
        // Evitar voltear completamente la cámara
        camera.pitch = camera.pitch.clamp(-1.45, 1.45);
    }

    // BOTÓN IZQUIERDO Arrastrar
    if mouse_buttons.pressed(MouseButton::Left) {
        let right = transform.rotation * Vec3::X;
        let up = transform.rotation * Vec3::Y;

        // Pan proporcional a la distancia.
        let scale = camera.distance * camera.pan_sensitivity;
        camera.focus += (-right * delta.x + up * delta.y) * scale;
    }

    // RUEDA zoom
    let mut zoom = match mouse_scroll.unit {
        MouseScrollUnit::Line => mouse_scroll.delta.y,
        MouseScrollUnit::Pixel => mouse_scroll.delta.y * 0.02,
    };

    if keyboard_input.pressed(KeyCode::Equal) || keyboard_input.pressed(KeyCode::NumpadAdd) {
        zoom += 0.2;
    }
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
    
    // CALCULAR POSICIÓN
    let cos_pitch = camera.pitch.cos();

    let offset = Vec3::new(
        camera.distance * cos_pitch * camera.yaw.sin(),
        camera.distance * camera.pitch.sin(),
        camera.distance * cos_pitch * camera.yaw.cos(),
    );

    transform.translation = camera.focus + offset;
    transform.look_at(camera.focus, Vec3::Y);

}


pub fn camera_controls_help(
    mut contexts: EguiContexts,
) {
    let Ok(ctx) = contexts.ctx_mut() else { return; };

    egui::Area::new("camera_controls_help".into())
        .order(egui::Order::Foreground)
        .anchor(egui::Align2::LEFT_BOTTOM, [8.0, -8.0])
        .interactable(false)
        .show(ctx, |ui| {
            egui::Frame::window(ui.style())
                .fill(egui::Color32::from_rgba_premultiplied(5, 10, 18, 210))
                .show(ui, |ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;

                    let key_color = egui::Color32::from_rgb(47, 128, 237);
                    let text_color = egui::Color32::from_rgb(180, 190, 205);

                    let key = |text: &str| {
                        egui::RichText::new(text)
                            .color(key_color)
                            .strong()
                            .monospace()
                            .size(10.0)
                    };

                    let text = |text: &str| {
                        egui::RichText::new(text)
                            .color(text_color)
                            .size(10.0)
                    };

                    ui.horizontal(|ui| {
                        ui.label(key("Scroll | +/-"));
                        ui.label(text("Zoom"));

                        ui.separator();

                        ui.label(key("L-Drag"));
                        ui.label(text("Move"));

                        ui.separator();

                        ui.label(key("R-Drag"));
                        ui.label(text("Rotate"));

                        ui.separator();

                        ui.label(key("0"));
                        ui.label(text("Reset"));

                        ui.separator();

                        ui.label(key("Clic"));
                        ui.label(text("Copy"));

                        ui.separator();

                        ui.label(key("2Clic"));
                        ui.label(text("Center"));
                    });
                });
        });
}


pub fn zoom(
    mut commands: Commands,
    mut camera_last_position: ResMut<visualizer::resource::camera::LastPosition>,
    time: Res<Time>,
    mut camera_query: Query<(
            Entity, &mut visualizer::component::world_camera::WorldCamera,
            &mut Transform, 
            &mut visualizer::component::world_camera::Zoom
        ),With<Camera3d>,
    >,
) {
    let Ok((camera_entity, mut camera, mut transform, mut zoom)) = camera_query.single_mut() else {
        return;
    };

    if zoom.elapsed == 0.0 {
        camera_last_position.position= zoom.start_focus;
        camera_last_position.distance= zoom.start_distance;
    }
    let duration = 1.5; // segundos, fijo para todos

    zoom.elapsed += time.delta_secs();
    let t = (zoom.elapsed / duration).clamp(0.0, 1.0);

    // Suavizado (smoothstep)
    let eased = t * t * (3.0 - 2.0 * t);

    camera.focus = zoom.start_focus.lerp(zoom.target, eased);
    camera.distance = zoom.start_distance.lerp(zoom.target_distance, eased);
    camera.yaw = 0.0;
    camera.pitch = 0.0;

    // Posición orbital
    let cos_pitch = camera.pitch.cos();
    let offset = Vec3::new(
        camera.distance * cos_pitch * camera.yaw.sin(),
        camera.distance * camera.pitch.sin(),
        camera.distance * cos_pitch * camera.yaw.cos(),
    );
    transform.translation = camera.focus + offset;
    transform.look_at(camera.focus, Vec3::Y);

    if t >= 1.0 {
        commands.entity(camera_entity).remove::<visualizer::component::world_camera::Zoom>();
    }

}
