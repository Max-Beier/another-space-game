use bevy::{
    prelude::{shape, Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Vec3},
    transform::TransformBundle,
};
use bevy_rapier3d::prelude::{Ccd, GravityScale, RigidBody, Sleeping};
use rand::rngs::StdRng;

use crate::{bundles::CBBundle, components::CBClass};

pub fn generate_star(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut _rng: &StdRng,
    _position: &Vec3,
) {
    let cb = CBBundle {
        class: CBClass::Star,
        ..Default::default()
    };

    let mesh: PbrBundle = PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 10.0,
                subdivisions: 10,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::YELLOW,
            ..Default::default()
        }),
        ..Default::default()
    };

    commands
        .spawn(RigidBody::Dynamic)
        .insert(TransformBundle::default())
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(GravityScale(0.0))
        .insert(cb)
        .insert(mesh);
}
