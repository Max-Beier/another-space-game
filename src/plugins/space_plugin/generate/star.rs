use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Vec3,
};

use rand::{rngs::StdRng, Rng};

use crate::{
    bundles::CBBundle,
    components::{CBClass, CBInitialVelocity, CBName, CBRadius, CBSpin, CBSurfaceGravity},
    resources::Space,
};

use super::celestial_body::generate_celestial_body;

pub fn generate_star(
    commands: &mut Commands,
    space: &Space,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    rng: &mut StdRng,
    position: Vec3,
) -> f32 {
    let cb = CBBundle {
        name: CBName("Star".to_string()),
        class: CBClass::Star,
        radius: CBRadius(rng.gen_range(space.star_radius.clone())),
        surface_gravity: CBSurfaceGravity(rng.gen_range(space.star_surface_gravity.clone())),
        initial_velocity: CBInitialVelocity::default(),
        spin: CBSpin(10.0),
    };

    let star_mesh: PbrBundle = PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: cb.radius.0,
                subdivisions: space.subdivisions.clone(),
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

    generate_celestial_body(commands, cb.clone(), star_mesh, position, None);

    cb.surface_gravity.0 * cb.radius.0 * cb.radius.0 / 6.674e-11
}
