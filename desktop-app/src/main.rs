mod ui;
mod core;
use eframe::egui;
use bevy::prelude::*;

use crate::{core::system, ui::windows::workspace::workspace_window::WorkspaceWindow};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 && args[1] == "--visualizer" {
        run_visualizer_3d();
        Ok(())
    } else {
        let icon = ui::utilities::load_favicon();
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([1000.0, 650.0])
                .with_icon(icon),
            ..Default::default()
        };

        system::setup::init_config(None);

        eframe::run_native(
            "WorldServers",
            options,
            Box::new(WorkspaceWindow::create)
        ).unwrap();
        Ok(())
    }
    
}



fn run_visualizer_3d() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_cube)
        .add_systems(Update, rotate_cube)
        .run();
    std::process::exit(0);
}

fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.6, 0.9))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_cube(
    mut cubes: Query<&mut Transform, With<Mesh3d>>,
    time: Res<Time>,
) {
    for mut transform in &mut cubes {
        transform.rotation *= Quat::from_rotation_y(time.delta_secs() * 0.5);
        transform.rotation *= Quat::from_rotation_x(time.delta_secs() * 0.3);
    }
}