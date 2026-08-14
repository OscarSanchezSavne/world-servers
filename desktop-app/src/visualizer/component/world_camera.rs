use bevy::prelude::*;

#[derive(Component)]
pub struct WorldCamera {
    pub focus: Vec3,

    pub distance: f32,

    pub yaw: f32,
    pub pitch: f32,

    pub rotate_sensitivity: f32,
    pub pan_sensitivity: f32,
    pub zoom_sensitivity: f32,

    pub min_distance: f32,
    pub max_distance: f32,
}

impl WorldCamera {
    pub fn new(
        focus: Vec3, distance: f32,
    ) -> Self {
        Self {
            focus, distance, 
            yaw: 0.0,
            pitch: 0.0,

            rotate_sensitivity: 0.005,
            pan_sensitivity: 0.0015,
            zoom_sensitivity: 0.12,

            min_distance: 10.0,
            max_distance: 600.0,
        }
    }
}


#[derive(Component)]
pub struct Zoom {
    pub target: Vec3,
    pub target_distance: f32,
    pub start_focus: Vec3,
    pub start_distance: f32,
    pub elapsed: f32,
}

impl Zoom {
}