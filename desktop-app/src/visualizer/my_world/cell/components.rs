use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Cell{
    pub center: Vec3,
    pub size: f32,
    pub external: bool,
}



#[derive(Component, Debug)]
pub enum CellState {
    Unassigned,
    Assigned,
    Processing,
    Failed,
    InLine,
}