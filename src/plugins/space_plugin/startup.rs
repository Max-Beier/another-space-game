use crate::resources::Space;

use super::generate::generate_space;
use bevy::{
    prelude::{Assets, Commands, Mesh, Res, ResMut, StandardMaterial},
    scene::SceneBundle,
};

pub fn startup(
    mut commands: Commands,
    space: Res<Space>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(SceneBundle::default());
    generate_space(commands, space, meshes, materials, 100, 19);
}
