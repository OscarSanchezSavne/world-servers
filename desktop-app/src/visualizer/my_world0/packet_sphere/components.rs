use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component)]
pub struct PacketSphere{
    pub size: f32,
    pub source: Vec3,
    pub target: Vec3,
}