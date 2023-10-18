use std::ops::Range;

use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Vec3,
};
use rand::{rngs::StdRng, Rng};

use crate::{
    bundles::CBBundle,
    components::{CBClass, CBInitialVelocity, CBName, CBRadius, CBSpin, CBSurfaceGravity},
};

use super::celestial_body::generate_celestial_body;

// CONST
const PLANET_RADIUS: Range<f32> = 100.0..4000.0;
const PLANET_SURFACE_GRAVITY: Range<f32> = 4.0..20.0;

pub fn generate_planet(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rng: &mut StdRng,
    position: Vec3,
    center_mass: (Vec3, f32),
) {
    let cb = CBBundle {
        name: CBName("Planet".to_string()),
        class: CBClass::Planet,
        radius: CBRadius(rng.gen_range(PLANET_RADIUS)),
        surface_gravity: CBSurfaceGravity(rng.gen_range(PLANET_SURFACE_GRAVITY)),
        initial_velocity: CBInitialVelocity(Vec3::new(100.0, 0.0, 0.0)),
        spin: CBSpin(10.0),
    };

    let mesh: PbrBundle = PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: cb.radius.0,
                subdivisions: 10,
            })
            .unwrap(),
        ),
        material: materials.add(StandardMaterial {
            unlit: true,
            base_color: Color::GREEN,
            ..Default::default()
        }),
        ..Default::default()
    };

    generate_celestial_body(commands, cb, mesh, position, Some(center_mass));
}
