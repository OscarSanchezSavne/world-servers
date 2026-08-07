use bevy::prelude::*;

use crate::visualizer::my_world0::packet_sphere::components::PacketSphere;


pub fn run(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut PacketSphere)>,
)
{
    let units = 6.0; // unidades por segundo

    for (entity, mut transform, packet) in query.iter_mut() {
        let direction = (packet.target - packet.source).normalize_or_zero();
        let step = direction * units * time.delta_secs();

        transform.translation += step;
        let to_target = packet.target - packet.source;
        let traveled = transform.translation - packet.source;

        // Verificar si ya pasó al target
        if traveled.length_squared() >= to_target.length_squared() {
            commands.entity(entity).despawn();
        }
    }
}