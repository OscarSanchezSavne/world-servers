use bevy::ecs::resource::Resource;

#[derive(Resource, Default)]
pub struct LogBuffer {
    lines: Vec<String>,
    pub minimized: bool,
}

impl LogBuffer {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.lines.push(msg.into());
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.lines.iter()
    }
}
