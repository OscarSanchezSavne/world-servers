use bevy::prelude::*;

#[derive(Component, Clone, PartialEq)]
pub struct Cell{
    pub center: Vec3,
    pub size: f32,
    pub external: bool,
}



#[derive(Component, Debug, PartialEq)]
pub enum CellState {
    Unassigned,
    Assigned,
    Processing,
    Failed,
    InLine,
}