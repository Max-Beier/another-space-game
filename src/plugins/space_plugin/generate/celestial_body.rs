use bevy::{
    prelude::{Commands, PbrBundle, Transform, Vec3},
    transform::TransformBundle,
};
use bevy_rapier3d::prelude::Collider;

use crate::components::CelestialBody;

pub fn generate_celestial_body(
    commands: &mut Commands,
    cb: CelestialBody,
    mesh: PbrBundle,
    position: Vec3,
) {
    commands
        .spawn(Collider::ball(cb.radius))
        .insert(cb)
        .insert(mesh)
        .insert(TransformBundle {
            local: Transform::from_xyz(position.x, position.y, position.z),
            ..Default::default()
        });
}
