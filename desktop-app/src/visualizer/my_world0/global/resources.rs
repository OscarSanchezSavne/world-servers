use std::{collections::HashMap, sync::{Mutex, mpsc}};

use bevy::prelude::*;
use uuid::Uuid;

use crate::{core::server::manager::Server, ui::utilities::ExecutionState};


#[derive(Resource)]
pub struct WorldData {
    pub cell_width: f32,
    pub cell_depth: f32,
    pub cells: Vec<Entity>,
    pub index_cells_sort: Vec<usize>,
    pub server_event_receiver: Mutex<mpsc::Receiver<ExecutionState>>,
    pub server_event_sender: mpsc::Sender<ExecutionState>,
    pub server_cell_entity_map: HashMap<Uuid, Entity>,
    pub server_entity_map: HashMap<Uuid, Entity>,
    pub servers: Vec<Server>,
}

impl Default for WorldData
{
    fn default() -> Self {
        let (sender, receiver) = std::sync::mpsc::channel::<ExecutionState>();
        WorldData {
            cell_width: 9.9,
            cell_depth: 18.0,
            cells: Vec::new(),
            index_cells_sort: Vec::new(),
            server_event_receiver: Mutex::new(receiver),
            server_event_sender: sender,
            server_cell_entity_map: HashMap::new(),
            server_entity_map: HashMap::new(),
            servers: Vec::new()
        }
    }
}



impl WorldData{
    pub fn get_server(&self, uuid: Uuid) -> Server {
        self.servers.iter()
            .find(|s| s.uuid == Some(uuid))
            .unwrap()
            .clone()
    }
}