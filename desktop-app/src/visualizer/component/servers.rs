use bevy::{ecs::component::Component, math::Vec3};
use uuid::Uuid;

use crate::{core, visualizer};

#[derive(Component, Debug, Default)]
pub struct Servers{
    pub list: Vec<visualizer::component::server::Server>,
    pub list_original_servers: Vec<core::server::manager::Server>,
}


impl Servers {
    
    pub fn create() -> Self {
        let list_original_servers= core::server::manager::Server::get_servers();
        let mut bevy_servers= Vec::new();
        for server in list_original_servers.iter() {
            bevy_servers.push(visualizer::component::server::Server{
                ip: server.server_ip.clone(),
                name: server.server_name.clone(),
                external: false,
                uuid: server.uuid.clone(),
                entity: None,
                entity_cell: None,
                position: Vec3::ZERO,
                visible: true,
                entity_disk_load: None,
                entity_cpu_load: None,
                entity_ram_load: None,
            });
        }
        Self { 
            list: bevy_servers,
            list_original_servers 
        }
    }
    
    pub fn add_external_server(
        &mut self, ip: String, name: String
    ){
        if self.list.iter().any(|server| server.ip == ip) {
            return;
        }
        self.list.push(visualizer::component::server::Server{
            ip: ip,
            name: name,
            external: true,
            uuid: Some(Uuid::new_v4()),
            entity: None,
            entity_cell: None,
            position: Vec3::ZERO,
            visible: true,
                entity_disk_load: None,
                entity_cpu_load: None,
                entity_ram_load: None,
        });
    }

}   