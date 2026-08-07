use bevy::ecs::resource::Resource;

use crate::core::server::manager::Server;

#[derive(Resource, Debug)]
pub struct ServersData {
    pub servers: Vec<Server>,
    pub len_servers: usize,
}

impl Default for ServersData {
    fn default() -> Self {
        let servers= Server::get_servers();
        Self {
            len_servers: servers.len(),
            servers: Server::get_servers(),
        }
    }
}