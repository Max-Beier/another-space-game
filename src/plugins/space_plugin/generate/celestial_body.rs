use bevy::{
    pbr::wireframe::Wireframe,
    prelude::{Commands, PbrBundle, Transform, Vec3},
    transform::TransformBundle,
};
use bevy_rapier3d::prelude::{Collider, ColliderMassProperties, RigidBody, Sleeping, Velocity};

use crate::bundles::CBBundle;

pub fn generate_celestial_body(
    commands: &mut Commands,
    mut cb: CBBundle,
    mesh: PbrBundle,
    position: Vec3,
    center_mass: Option<(Vec3, f32)>,
) {
    let mass = cb.surface_gravity.0 * cb.radius.0 * cb.radius.0 / 6.674e-11;

    // Stable Orbit
    if let Some(center_mass) = center_mass {
        let orbit_radius = (center_mass.0 - position).length();
        let orbit_velocity_direction = center_mass
            .0
            .normalize_or_zero()
            .cross(position)
            .normalize_or_zero();
        let orbit_velocity = (6.674e-11 * center_mass.1 / orbit_radius).sqrt();
        cb.initial_velocity.0 = orbit_velocity_direction * orbit_velocity;
    }

    commands
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(cb.radius.0))
        .insert(ColliderMassProperties::Mass(mass))
        .insert(Velocity::linear(cb.initial_velocity.0))
        .insert(cb)
        .insert(mesh)
        .insert(Wireframe)
        .insert(TransformBundle {
            local: Transform::from_translation(position),
            ..Default::default()
        });
}
