use bevy::prelude::*;

use crate::{visualizer::my_world0::{self, global::components::ModelReady}};

pub fn capture(
    mut commands: Commands,
    query: Query<
        ( 
            Entity, 
            &my_world0::cell::components::CellState,
            &my_world0::server::components::ServerModel
        ),
        (
            With<my_world0::cell::components::Cell>,
            With<ModelReady>,
            Changed<my_world0::cell::components::CellState>
        ),
    >,
    world_data: Res<my_world0::global::resources::WorldData>,
    traffic_channel: ResMut<my_world0::server::resources::TrafficChannel>,
) 
{

    let servers = world_data.servers.clone();
    for (cell_entiy, state, server) in &query {

        if *state != my_world0::cell::components::CellState::InLine{
            continue;
        }
        let server= servers.iter().find(|s| s.uuid == Some(server.uuid)).unwrap();
        
        let tx= traffic_channel.tx.clone();
        server.clone().async_run_tcpdump(tx);

        commands.entity(cell_entiy).insert(my_world0::cell::components::CellState::Processing);

    }

}
