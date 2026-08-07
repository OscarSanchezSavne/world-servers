use bevy::{ecs::{component::Component, entity::Entity}, math::Vec3};
use uuid::Uuid;



#[derive(Component, Debug, Clone)]
pub struct Cell{
    pub position: Vec3,
    pub asigned: bool,
    pub uuid: Uuid,
    pub entity: Option<Entity>,
}

#[derive(Component)]
pub enum CellType {
    External,
    Internal,
}


#[derive(Component)]
pub struct PulsingCell {
    pub stop: bool,
    pub elapsed: f32,
}

impl PulsingCell {
    pub fn start() -> Self {
        Self {
            stop: false,
            elapsed: 0.0,
        }
    }

    pub fn stop() -> Self {
        Self {
            stop: true,
            elapsed: 0.0,
        }
    }
}