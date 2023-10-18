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
const STAR_RADIUS: Range<f32> = 5000.0..10000.0;
const STAR_SURFACE_GRAVITY: Range<f32> = 30.0..50.0;

pub fn generate_star(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rng: &mut StdRng,
    position: Vec3,
) -> f32 {
    let cb = CBBundle {
        name: CBName("Star".to_string()),
        class: CBClass::Star,
        radius: CBRadius(rng.gen_range(STAR_RADIUS)),
        surface_gravity: CBSurfaceGravity(rng.gen_range(STAR_SURFACE_GRAVITY)),
        initial_velocity: CBInitialVelocity::default(),
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
            base_color: Color::YELLOW,
            ..Default::default()
        }),
        ..Default::default()
    };

    generate_celestial_body(commands, cb.clone(), mesh, position, None);

    cb.surface_gravity.0 * cb.radius.0 * cb.radius.0 / 6.674e-11
}
