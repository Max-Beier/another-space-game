use bevy::prelude::{Assets, Commands, Mesh, Res, ResMut, StandardMaterial, Vec3};

use crate::resources::Space;

use self::starsystem::generate_star_system;

mod celestial_body;
mod planet;
mod star;
mod starsystem;

pub fn generate_space(
    commands: Commands,
    space: Res<Space>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
    _radius: u32,
    base_seed: u32,
) {
    // TODO: use radius to loop through 3d space and the seed to determine the positions of star systems
    generate_star_system(commands, space, meshes, materials, base_seed, Vec3::ZERO);
}
