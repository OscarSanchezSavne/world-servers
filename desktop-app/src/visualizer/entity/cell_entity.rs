use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::visualizer;

pub fn create(
    commands: &mut Commands,
    asset_server: &AssetServer,
    cell: visualizer::component::cell::Cell
)->Entity
{
    let hex_handle: Handle<WorldAsset> = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset("gltf/hexagono.gltf"),
    );


    let mut new_cell_component= cell.clone();
    let entity= commands.spawn((
        WorldAssetRoot(hex_handle.clone()),
        Transform {
            translation: cell.position,
            scale: Vec3::splat(1.0),
            ..default()
        },
    )).observe(click_handler).id();
    
    new_cell_component.entity= Some(entity);
    commands.entity(entity).insert(new_cell_component);

    entity
}

fn click_handler(
    click: On<Pointer<Click>>,
    cells: Query<&visualizer::component::cell::Cell>,
    query_servers: Query<&visualizer::component::server::Server>,
    mut commands: Commands,
    mut contexts: EguiContexts,
    mut toaster: ResMut<visualizer::resource::toaster::Toaster>,
    mut camera_query: Query<
        (Entity, &mut visualizer::component::world_camera::WorldCamera),
        With<Camera3d>,
    >,
) {

    if click.count >= 2 {

        let Ok((entity_camera, camera)) = camera_query.single_mut() else {
            return;
        };

        let Ok(cell) = cells.get(click.entity) else {
            return;
        };

        commands.entity(entity_camera).insert(
            visualizer::component::world_camera::Zoom{
                target: Vec3 { x:  cell.position.x + 5.0, y:  cell.position.y - 6.0, z: cell.position.z},
                target_distance: 25.0,
                start_focus: camera.focus,
                start_distance: camera.distance,
                elapsed: 0.0,
            }
        );

        return;
    }

    let Ok(cell) = cells.get(click.entity) else {
        return;
    };

    let Some(server)= query_servers.iter().find(|ser|{
        ser.entity_cell == cell.entity
    }) else{
        return;
    };

    let Ok(ctx) = contexts.ctx_mut() else { return; };
    ctx.copy_text(format!("Name: {}\nIp: {}", server.name, server.ip));

    toaster.add("Copied".to_string());
    
}
