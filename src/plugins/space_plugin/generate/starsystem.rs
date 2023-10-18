use bevy::prelude::{Assets, Commands, Mesh, ResMut, StandardMaterial, Vec3};
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::star::generate_star;

// CONST
const MIN_PLANETS_IN_STAR_SYSTEM: usize = 1;
const MAX_PLANETS_IN_STAR_SYSTEM: usize = 10;

pub fn generate_star_system(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    position: &Vec3,
    starsystem_seed: u32,
) {
    let mut rng: StdRng = StdRng::seed_from_u64(starsystem_seed as u64);
    generate_star(commands, meshes, materials, &rng, position);

    for planet_index in MIN_PLANETS_IN_STAR_SYSTEM
        ..rng.gen_range(MIN_PLANETS_IN_STAR_SYSTEM..MAX_PLANETS_IN_STAR_SYSTEM)
    {
        // TODO: PROPER DISTANCE; PROPER RADIUS
    }
}
