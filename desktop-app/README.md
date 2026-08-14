Commands

Asegurarse que el usuario si ejecuta tcpdump sin clave

    Sudo sin contraseña solo para tcpdump (en /etc/sudoers):
    tu_usuario ALL=(ALL) NOPASSWD: /usr/sbin/tcpdump

cd desktop-app2 
cargo run
cargo test
cargo test test_clean_config
cargo test -- --no-capture test_clean_config 

docker run --rm --name ssh-test -p 2222:22 rastasheep/ubuntu-sshd:18.04
Clave root:root

-- Configurar contenedor
docker exec -it ssh-test bash
apt-get update
apt-get install -y tcpdump iproute2
apt-get install -y sudo
sudo tcpdump -i $(ip -o -4 route show to default | awk '{print $5}') -nn -tt -l not port 22


AGREGA LA OPCIÓN QUE PREGUNTE SI HACE LOGIN POR LLAVE SSH O CONTRASEÑA
AL FORM AGREGAR LOS CAMPOS PUERTO

cargo run
cargo run -- --visualizer

Camara por defecto


    .add_plugins(FreeCameraPlugin)


    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 100.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::NEG_Z),
        FreeCamera::default(),
    ));


Trazar lineas 


 * 
    gizmos.line(
        Vec3::new(-5.0, 0.51, -4.5),  // inicio
        Vec3::new(5.0, 0.51, -4.5), // fin
        Color::srgb(1.0, 0.0, 0.0), // color rojo
    );
    gizmos.line(
        Vec3::new(5.0, 0.51, -4.5),  // inicio
        Vec3::new(5.0, 0.51, 4.5), // fin
        Color::srgb(1.0, 0.0, 0.0), // color rojo
    );

    gizmos.line(
        Vec3::new(5.0, 0.51, 4.5),  // inicio
        Vec3::new(-5.0, 0.51, 4.5), // fin
        Color::srgb(1.0, 0.0, 0.0), // color rojo
    );

    gizmos.line(
        Vec3::new(-5.0, 0.51, 4.5),  // inicio
        Vec3::new(-5.0, 0.51, -4.5), // fin
        Color::srgb(1.0, 0.0, 0.0), // color rojo
    );



Ver elementos de un objeto gtlf

pub fn update(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_data: ResMut<WorldData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hexagon_query: Query<&mut Hexagon>,

    children_query: Query<&Children>,
    material_name_query: Query<&GltfMaterialName>,
    mut material_query: Query<&mut MeshMaterial3d<StandardMaterial>>,
    name_query: Query<&Name>,
) {
    if world_data.update_cells.is_empty() {
        return;
    }

    //let update_cells = std::mem::take(&mut world_data.update_cells);
    let update_cells = world_data.update_cells.clone();

    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.0, 0.0),
        ..default()
    });

    println!("for 1");
    for update_cell_entity_id in update_cells {
        println!("for 2 ");
        for descendant in children_query.iter_descendants(update_cell_entity_id) {
            let name = name_query
                .get(descendant)
                .map(|n| n.as_str())
                .unwrap_or("SIN_NAME");

            let has_material = material_query.get(descendant).is_ok();

            let material_name = material_name_query
                .get(descendant)
                .map(|m| m.0.as_str())
                .unwrap_or("SIN_GLTF_MATERIAL_NAME");

            println!(
                "descendant: {:?} | name: {} | has_material: {} | gltf_material: {}",
                descendant,
                name,
                has_material,
                material_name
            );
        }
    }
}

