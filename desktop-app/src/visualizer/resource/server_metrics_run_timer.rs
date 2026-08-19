use std::sync::{Mutex, mpsc};

use bevy::{ecs::resource::Resource, time::{Timer, TimerMode}};

use crate::ui::utilities::ServerMetrics;

#[derive(Resource)]
pub struct ServerMetricsRunTimer{
    pub timer: Timer,
    pub tx: mpsc::Sender<ServerMetrics>,
    pub rx: Mutex<mpsc::Receiver<ServerMetrics>>,
}

impl ServerMetricsRunTimer {

    pub fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel::<ServerMetrics>();
        Self{
            timer: Timer::from_seconds(30.0, TimerMode::Repeating),
            tx, 
            rx: Mutex::new(rx),
        }
    }


}
