use bevy::{
    prelude::{
        shape, Assets, Color, Commands, Mesh, PbrBundle, PointLight, ResMut, StandardMaterial,
        Transform, Vec3,
    },
    scene::SceneBundle,
};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(SceneBundle::default());

    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
            material: materials.add(Color::ALICE_BLUE.into()),
            transform: Transform {
                translation: Vec3::new(0.0, -5.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        PointLight::default(),
    );

    commands.spawn(floor);
}
