use bevy::prelude::{Assets, Commands, Mesh, Res, ResMut, StandardMaterial, Vec3};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::resources::Space;

use super::{planet::generate_planet, star::generate_star};

pub fn generate_star_system(
    mut commands: Commands,
    space: Res<Space>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    starsystem_seed: u32,
) {
    let mut rng: StdRng = StdRng::seed_from_u64(starsystem_seed as u64);
    let center_mass = generate_star(
        &mut commands,
        &space.clone(),
        &mut meshes,
        &mut materials,
        &mut rng,
        position,
    );

    for _ in space.planets_count.start.clone()..rng.gen_range(space.planets_count.clone()) {
        let planet_position = Vec3::new(
            rng.gen_range(space.planets_distance.clone()),
            rng.gen_range(space.planets_distance.clone()),
            rng.gen_range(space.planets_distance.clone()),
        );

        generate_planet(
            &mut commands,
            &space,
            &mut meshes,
            &mut materials,
            &mut rng,
            planet_position,
            (position, center_mass),
        );
    }
}
