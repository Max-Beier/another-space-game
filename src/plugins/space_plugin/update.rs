use bevy::{
    prelude::{
        BuildChildren, Commands, DirectionalLight, Entity, Quat, Query, Res, Transform, Vec3, With,
        Without,
    },
    time::Time,
};
use bevy_rapier3d::prelude::{Collider, Velocity};

use crate::components::{CBClass, CelestialBody, PlayerController};

const G: f32 = 6.67430e-11;

pub fn foating_point_origin(
    mut transforms: Query<
        &mut Transform,
        (
            Without<PlayerController>,
            With<Collider>,
            With<CelestialBody>,
        ),
    >,
    mut p_transform_q: Query<&mut Transform, With<PlayerController>>,
) {
    let mut p_transform = p_transform_q.single_mut();

    let origin_offset = p_transform.translation.length();
    if origin_offset.abs() > 1000.0 {
        for mut transform in transforms.iter_mut() {
            transform.translation -= p_transform.translation;
        }
        p_transform.translation = Vec3::ZERO;
    }
}

pub fn update(
    mut commands: Commands,
    time: Res<Time>,
    mut cbs_q: Query<(Entity, &mut Transform, &mut CelestialBody), Without<PlayerController>>,
    mut player_q: Query<(Entity, &mut Transform, &mut Velocity, &PlayerController)>,
    mut light_q: Query<
        (&mut Transform, &mut DirectionalLight),
        (
            Without<PlayerController>,
            Without<Velocity>,
            Without<CelestialBody>,
        ),
    >,
) {
    let (p_entity, mut p_transform, mut p_velocity, p_controller) = player_q.single_mut();
    let mut c_force = Vec3::ZERO;
    let mut c_sqr_light_distance = f32::MAX;

    for (cb_entity, mut cb_transform, mut cb) in cbs_q.iter_mut() {
        // Update Orbits
        if let Some(orbit) = &cb.orbit {
            cb_transform.rotate_around(
                orbit.center_origin,
                Quat::from_rotation_y(orbit.velocity * time.delta_seconds()),
            );

            cb.atmosphere.center = cb_transform.translation;
        }

        // Update Forces
        let cb_mass = cb.surface_gravity * cb.radius * cb.radius / G;
        let sqr_dist = (cb_transform.translation - p_transform.translation).length_squared();
        let force_dir = (cb_transform.translation - p_transform.translation).normalize_or_zero();
        let force: Vec3 = (force_dir * G * cb_mass) / sqr_dist;

        // Only one body can affect the player at a time
        if force.length_squared() > c_force.length_squared() {
            let gravity_up = -force_dir.normalize_or_zero();
            p_transform.rotation =
                Quat::from_rotation_arc(p_transform.local_y(), gravity_up) * p_transform.rotation;

            if p_controller.is_colliding {
                commands.entity(cb_entity).add_child(p_entity);
            } else {
                p_velocity.linvel = force * time.delta_seconds();
                commands.entity(p_entity).remove_parent();
            }

            c_force = force;
        }

        // Update Light
        if matches!(cb.class, CBClass::Star) && sqr_dist < c_sqr_light_distance {
            let (mut l_transform, mut _light) = light_q.single_mut();
            let l_dir = (cb_transform.translation - l_transform.translation).normalize_or_zero();
            cb.atmosphere.light_dir = l_dir;

            l_transform.rotation =
                Quat::from_rotation_arc(l_transform.local_z(), l_dir) * l_transform.rotation;

            c_sqr_light_distance = sqr_dist;
        }
    }
}
