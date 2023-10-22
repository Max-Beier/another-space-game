use bevy::prelude::{
    shape, Assets, Color, Commands, Mesh, PbrBundle, ResMut, StandardMaterial, Vec3,
};

use rand::{rngs::StdRng, Rng};

use crate::{
    components::{CBClass, CelestialBody},
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
) {
    let cb = CelestialBody {
        name: "Star".to_string(),
        class: CBClass::Star,
        radius: rng.gen_range(space.star_radius.clone()),
        surface_gravity: rng.gen_range(space.star_surface_gravity.clone()),
        spin_velocity: 0.0,
        orbit: None,
        atmosphere: None,
    };

    let star_mesh: PbrBundle = PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: cb.radius,
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

    generate_celestial_body(commands, cb.clone(), star_mesh, position);
}
