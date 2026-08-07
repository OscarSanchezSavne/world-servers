use bevy::ecs::system::Single;
use bevy::prelude::*;
use bevy::text::{FontSize, TextLayoutInfo};


use crate::visualizer::my_world;


pub fn update_positions(
    camera: Single<(&Camera, &GlobalTransform)>,
    servers: Query<&GlobalTransform, With<my_world::component::server::Server>>,
    mut labels: Query<(
        &my_world::component::server::Label, &mut Node, &mut TextFont, &TextLayoutInfo
    )>,
) {
    let (camera, camera_transform) = *camera;

    for (label, mut node, mut text_font, layout) in &mut labels {
        let Ok(server_transform) = servers.get(label.server) else {
            continue;
        };

        let world_position =
            server_transform.translation()
            + Vec3::new(0.0, 0.5, 0.0);

        let Ok(screen_position) =
            camera.world_to_viewport(
                camera_transform,
                world_position,
            )
        else {
            continue;
        };
        
        let text_size = layout.size; 
        let camera_distance = camera_transform.translation().z.abs() / 35.0;
        let scale = 1.0 / camera_distance.clamp(0.5, 3.0);

        let new_size = (14.0 * scale).clamp(6.0, 14.0);
        text_font.font_size = FontSize::Px(new_size);

        node.left = px(screen_position.x - scale - (text_size.x/2.2));
        node.top = px(screen_position.y - (20.0 * scale));

    }

}