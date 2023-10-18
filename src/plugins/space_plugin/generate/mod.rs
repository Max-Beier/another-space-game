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
    player_position: Vec3,
    _radius: u32,
    _base_seed: u32,
) {
    // TODO: use radius to loop through 3d space and the seed to determine the positions of star systems
    generate_star_system(commands, space, meshes, materials, player_position, 21);
}
