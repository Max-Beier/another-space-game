use bevy::prelude::{
    shape, Assets, BuildChildren, Color, Commands, Material, Mesh, PbrBundle, ResMut,
    StandardMaterial, Vec3,
};

use crate::{bundles::CBBundle, components::CBClass};

pub fn generate_space(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    player_position: &Vec3,
    radius: u32,
    seed: u32,
) {
    generate_star_system(commands, meshes, materials, player_position, 19);
}

pub fn generate_star_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    center: &Vec3,
    seed: u32,
) {
    let star = CBBundle {
        class: CBClass::Star,
        ..Default::default()
    };

    let skin = PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 10.0,
                subdivisions: 10,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::LIME_GREEN,
            ..Default::default()
        }),
        ..Default::default()
    };

    let skin_entity = commands.spawn(skin).id();
    commands.spawn(star).add_child(skin_entity);
}
