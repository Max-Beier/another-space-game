use crate::resources::Space;

use super::generate::generate_space;
use bevy::{
    prelude::{
        AmbientLight, Assets, Commands, DirectionalLight, DirectionalLightBundle, Mesh, Res,
        ResMut, StandardMaterial,
    },
    scene::SceneBundle,
};

pub fn startup(
    mut commands: Commands,
    space: Res<Space>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    mut ambient_light: ResMut<AmbientLight>,
) {
    // Scene
    commands.spawn(SceneBundle::default());

    // Light
    ambient_light.brightness = 0.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    // Space Generation
    generate_space(commands, space, meshes, materials, 100, 1905);
}
