use std::sync::{Mutex, mpsc};

use bevy::ecs::resource::Resource;

use crate::ui::utilities::ServerTraffic;

#[derive(Resource)]
pub struct TrafficChannel {
    pub tx: mpsc::Sender<ServerTraffic>,
    pub rx: Mutex<mpsc::Receiver<ServerTraffic>>,
}

impl Default for TrafficChannel
{
    fn default()-> Self
    {
        let (tx, rx) = std::sync::mpsc::channel::<ServerTraffic>();
        TrafficChannel{
            tx, 
            rx:Mutex::new(rx)
        }
    }
}