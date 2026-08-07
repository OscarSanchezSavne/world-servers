use bevy::ecs::component::Component;
use eframe::wgpu::naga::compact::KeepUnused::No;

use crate::{core, visualizer::my_world};

#[derive(Component, Debug, Default)]
pub struct Servers{
    pub list: Vec<my_world::component::server::Server>,
    pub list_original_servers: Vec<core::server::manager::Server>,
}


impl Servers {
    
    pub fn create() -> Self {
        let list_original_servers= core::server::manager::Server::get_servers();
        let mut bevy_servers= Vec::new();
        for server in list_original_servers.iter() {
            bevy_servers.push(my_world::component::server::Server{
                ip: server.server_ip.clone(),
                name: server.server_name.clone(),
                external: false,
                uuid: server.uuid.clone(),
                entity: None,
                entity_cell: None,
            });
        }
        Self { 
            list: bevy_servers,
            list_original_servers 
        }
    }

}   