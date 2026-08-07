use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Component)]
pub struct ModelReady;

#[derive(Component)]
pub struct ModelNodes {
    pub by_name: HashMap<String, Entity>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Start,
    GlobalLoaded,
    GridLoaded,
}