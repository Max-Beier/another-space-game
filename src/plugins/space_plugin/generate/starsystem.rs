use bevy::prelude::{Assets, Commands, Mesh, Res, ResMut, StandardMaterial, Vec3};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{components::CBOrbit, resources::Space};

use super::{planet::generate_planet, star::generate_star};

pub fn generate_star_system(
    mut commands: Commands,
    space: Res<Space>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    starsystem_seed: u32,
    center_position: Vec3,
) {
    let mut rng: StdRng = StdRng::seed_from_u64(starsystem_seed as u64);
    generate_star(
        &mut commands,
        &space.clone(),
        &mut meshes,
        &mut materials,
        &mut rng,
        center_position,
    );

    for _ in space.planets_count.start.clone()..rng.gen_range(space.planets_count.clone()) {
        let planet_position = Vec3::new(
            rng.gen_range(space.planets_distance.clone()),
            rng.gen_range(space.planets_distance.clone()),
            rng.gen_range(space.planets_distance.clone()),
        );

        let planet_orbit: CBOrbit = CBOrbit {
            center_origin: center_position,
            radius: (planet_position - center_position).length(),
            velocity: rng.gen_range(space.planet_orbit_velocity.clone()),
        };

        generate_planet(
            &mut commands,
            &space,
            &mut meshes,
            &mut materials,
            &mut rng,
            planet_position,
            planet_orbit,
        );
    }
}
