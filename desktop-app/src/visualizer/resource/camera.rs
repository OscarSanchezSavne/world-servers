use bevy::{ecs::resource::Resource, math::Vec3};

#[derive(Resource)]
pub struct LastPosition{
    pub position: Vec3,
    pub distance: f32,
}

impl Default for LastPosition 
{
    fn default()-> Self
    {
        Self{
            position: Vec3::ZERO,
            distance: 0.0
        }
    }
}