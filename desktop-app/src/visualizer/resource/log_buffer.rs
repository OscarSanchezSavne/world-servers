use std::sync::{Mutex, mpsc::{Receiver, Sender}};

use bevy::ecs::resource::Resource;

use crate::ui::utilities::ExecutionState;

#[derive(Resource)]
pub struct LogBuffer {
    lines: Vec<String>,
    pub minimized: bool,
    pub rx: Mutex<Receiver<ExecutionState>>,
    pub tx: Sender<ExecutionState>,
}

impl Default for LogBuffer {
    
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<ExecutionState>();
        Self { 
            lines: Vec::new(), 
            minimized: false, 
            rx: Mutex::new(rx), 
            tx: tx
        }
    }
}

impl LogBuffer {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.lines.push(msg.into());
    }
    
    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.lines.iter()
    }
}
