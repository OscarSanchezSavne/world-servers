use bevy::prelude::*;

#[derive(Resource)]
pub struct SphereAssets {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

pub fn setup_materials(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(SphereAssets {
        mesh: meshes.add(Sphere::new(0.5).mesh().ico(8).unwrap()),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 1.0),
            ..default()
        }),
    });
}