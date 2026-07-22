use bevy::prelude::*;

#[derive(Resource)]
pub struct CellMaterials {
    pub unassigned: Handle<StandardMaterial>,
    pub assigned: Handle<StandardMaterial>,
    pub processing: Handle<StandardMaterial>,
    pub failed: Handle<StandardMaterial>,
    pub inline: Handle<StandardMaterial>,

}

pub fn setup_materials(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(CellMaterials {
        unassigned: materials.add(StandardMaterial {
            base_color: Color::srgba(0.0, 0.0, 0.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
        assigned: materials.add(StandardMaterial {
            base_color: Color::srgb(0.2, 0.6, 1.0),
            ..default()
        }),
        processing: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 0.0, 1.0),
            ..default()
        }),
        failed: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        }),
        inline: materials.add(StandardMaterial {
            base_color: Color::srgb(0.0, 1.0, 0.0),
            ..default()
        }),
    });
}