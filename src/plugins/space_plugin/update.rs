use bevy::{
    prelude::{BuildChildren, Commands, Entity, Quat, Query, Res, Transform, Vec3, Without},
    time::Time,
};
use bevy_rapier3d::prelude::Velocity;

use crate::components::{CelestialBody, PlayerController};

const G: f32 = 6.67430e-11;

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut cbs_mut: Query<(Entity, &mut Transform, &CelestialBody), Without<PlayerController>>,

    mut player: Query<(Entity, &mut Transform, &mut Velocity, &PlayerController)>,
) {
    let (p_entity, mut p_transform, mut p_velocity, p_controller) = player.single_mut();
    let mut c_force = Vec3::ZERO;

    for (cb_entity, mut cb_transform, cb) in cbs_mut.iter_mut() {
        if let Some(orbit) = &cb.orbit {
            cb_transform.rotate_around(
                orbit.center_origin,
                Quat::from_rotation_y(orbit.velocity * time.delta_seconds()),
            );
        }

        let cb_mass = cb.surface_gravity * cb.radius * cb.radius / G;
        let sqr_dist = (cb_transform.translation - p_transform.translation).length_squared();
        let force_dir = (cb_transform.translation - p_transform.translation).normalize_or_zero();
        let force: Vec3 = (force_dir * G * cb_mass) / sqr_dist;

        // Only one body can affect the player at a time
        if force.length_squared() > c_force.length_squared() {
            let gravity_up = -force_dir.normalize_or_zero();
            let target_rotation =
                Quat::from_rotation_arc(p_transform.local_y(), gravity_up) * p_transform.rotation;

            if p_controller.is_colliding {
                commands.entity(cb_entity).add_child(p_entity);
            } else {
                commands.entity(p_entity).remove_parent();
                p_velocity.linvel = force * time.delta_seconds();
            }

            p_transform.rotation = target_rotation;
            c_force = force;
        }
    }
}
