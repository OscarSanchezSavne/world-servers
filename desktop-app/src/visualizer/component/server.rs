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
    pub entity_disk_load: Option<Entity>,
    pub entity_cpu_load: Option<Entity>,
    pub entity_ram_load: Option<Entity>,
    pub position: Vec3,
    pub visible: bool,
}


#[derive(Component, Debug, PartialEq)]
pub enum ServerState {
    Connecting,
    Inline,
    Error,
}


#[derive(Component, Default)]
pub struct ServerReady{}


#[derive(Component, Default)]
pub struct ServerReadyMetrics{}


#[derive(Component, Default)]
pub struct ObtainingServerMetrics{}


#[derive(Component, Default, Debug)]
pub struct ServerMetrics{
    pub cpu: f32,
    pub ram: f32,
    pub disk: f32,
}