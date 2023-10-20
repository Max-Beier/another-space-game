use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, Res, ResMut, StandardMaterial, Vec3,
};
use rand::{rngs::StdRng, Rng};

use crate::{
    bundles::CBBundle,
    components::{CBClass, CBInitialVelocity, CBName, CBRadius, CBSpin, CBSurfaceGravity},
    resources::Space,
};

use super::celestial_body::generate_celestial_body;

pub fn generate_planet(
    commands: &mut Commands,
    space: &Res<Space>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rng: &mut StdRng,
    position: Vec3,
    center_mass: (Vec3, f32),
) {
    let cb = CBBundle {
        name: CBName("Planet".to_string()),
        class: CBClass::Planet,
        radius: CBRadius(rng.gen_range(space.planet_radius.clone())),
        surface_gravity: CBSurfaceGravity(rng.gen_range(space.planet_surface_gravity.clone())),
        initial_velocity: CBInitialVelocity::default(),
        spin: CBSpin::default(),
    };

    let base_mesh = Mesh::try_from(shape::Icosphere {
        radius: cb.radius.0,
        subdivisions: space.subdivisions.clone(),
    })
    .unwrap();

    let planet_mesh: PbrBundle = PbrBundle {
        mesh: meshes.add(base_mesh),
        material: materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::GREEN,
            ..Default::default()
        }),
        ..Default::default()
    };

    generate_celestial_body(commands, cb, planet_mesh, position, Some(center_mass));
}
