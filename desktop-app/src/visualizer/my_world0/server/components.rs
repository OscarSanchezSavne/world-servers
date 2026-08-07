use bevy::prelude::*;
use uuid::Uuid;

#[derive(Component, Debug, PartialEq)]
pub struct ServerModel{
    pub uuid: Uuid
}