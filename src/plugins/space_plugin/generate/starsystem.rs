use std::ops::Range;

use bevy::prelude::{Assets, Commands, Mesh, ResMut, StandardMaterial, Vec3};
use rand::{rngs::StdRng, Rng, SeedableRng};

use super::{planet::generate_planet, star::generate_star};

// CONST
const STARSYSTEN_PLANETS: Range<usize> = 1..10;
const STARSYSTEM_PLANETS_DISTANCE: Range<f32> = 50000.0..100000.0;

pub fn generate_star_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    starsystem_seed: u32,
) {
    let mut rng: StdRng = StdRng::seed_from_u64(starsystem_seed as u64);
    let center_mass = generate_star(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut rng,
        position,
    );

    for i in STARSYSTEN_PLANETS.start..rng.gen_range(STARSYSTEN_PLANETS) {
        let x: f32 = rng.gen_range(STARSYSTEM_PLANETS_DISTANCE) * i as f32;
        let y: f32 = rng.gen_range(STARSYSTEM_PLANETS_DISTANCE) * i as f32;
        let z: f32 = rng.gen_range(STARSYSTEM_PLANETS_DISTANCE) * i as f32;

        generate_planet(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut rng,
            Vec3::new(x, y, z),
            (position, center_mass),
        );
    }
}
