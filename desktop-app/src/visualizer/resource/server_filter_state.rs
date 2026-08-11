use bevy::ecs::resource::Resource;

#[derive(Resource)]
pub struct ServerFilterState {
    pub minimized: bool,
}

impl Default for ServerFilterState {
    
    fn default() -> Self {
        Self { minimized: false }
    }

}