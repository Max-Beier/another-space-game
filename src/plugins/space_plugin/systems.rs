use super::generate::generate_space;
use bevy::{
    prelude::{Assets, Commands, Mesh, ResMut, StandardMaterial, Transform},
    scene::SceneBundle,
};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(SceneBundle::default());
    let player_transform = &Transform::default();
    generate_space(
        commands,
        meshes,
        materials,
        &player_transform.translation,
        100,
        19,
    );
}
