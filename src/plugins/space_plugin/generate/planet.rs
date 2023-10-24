use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, Res, ResMut, StandardMaterial, Vec3,
};
use rand::{rngs::StdRng, Rng};

use crate::{
    components::{AtmosphereSettings, CBClass, CBOrbit, CelestialBody},
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
    orbit: CBOrbit,
) {
    let cb_radius = rng.gen_range(space.planet_radius.clone());
    let cb = CelestialBody {
        name: "Planet".to_string(),
        class: CBClass::Planet,
        radius: cb_radius,
        surface_gravity: rng.gen_range(space.planet_surface_gravity.clone()),
        spin_velocity: 0.0,
        orbit: Some(orbit),
        atmosphere: AtmosphereSettings {
            center: position,
            ground_radius: cb_radius,
            ..Default::default()
        },
    };

    let base_mesh = Mesh::try_from(shape::Icosphere {
        radius: cb.radius,
        subdivisions: space.subdivisions.clone(),
    })
    .unwrap();

    let planet_mesh: PbrBundle = PbrBundle {
        mesh: meshes.add(base_mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::GREEN,

            ..Default::default()
        }),
        ..Default::default()
    };

    generate_celestial_body(commands, cb, planet_mesh, position);
}
