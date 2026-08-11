use bevy::{ecs::{component::Component, entity::Entity}, math::Vec3};
use uuid::Uuid;

#[derive(Component, Debug, Clone)]
pub struct Server{
    pub ip: String,
    pub name: String,
    pub external: bool,
    pub uuid: Option<Uuid>,
    pub entity: Option<Entity>,
    pub entity_cell: Option<Entity>,
    pub position: Vec3,
    pub visible: bool,
}


#[derive(Component, Debug, PartialEq)]
pub enum ServerState {
    Connecting,
    Inline,
    Error,
}


#[derive(Component)]
pub struct Label {
    pub server: Entity,
}


#[derive(Component, Default)]
pub struct ServerReady{}


