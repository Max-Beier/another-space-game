use bevy::{
    prelude::{Quat, Query, Res, Transform, Vec3, Without},
    time::Time,
};
use bevy_rapier3d::prelude::{Sleeping, Velocity};

use crate::components::{CBRadius, CBSpin, CBSurfaceGravity, PlayerController};

pub fn update(
    time: Res<Time>,
    mut cbs_mut: Query<
        (
            &Transform,
            &CBSurfaceGravity,
            &CBRadius,
            &CBSpin,
            &Sleeping,
            &mut Velocity,
        ),
        (Without<PlayerController>,),
    >,

    mut player: Query<(&mut Transform, &Sleeping, &mut Velocity, &PlayerController)>,
) {
    let mut forces = Vec::new();
    let mut p = player.single_mut();

    let mut c_force = Vec3::ZERO;
    for cb in cbs_mut.iter() {
        let mass = cb.1 .0 * cb.2 .0 * cb.2 .0 / 6.674e-11;
        for cb_current in cbs_mut.iter() {
            if (cb.0.translation != cb_current.0.translation) && !cb_current.4.sleeping {
                let sqr_dist = (cb.0.translation - cb_current.0.translation).length_squared();
                let force_dir = (cb.0.translation - cb_current.0.translation).normalize_or_zero();
                let force = (force_dir * 6.67430e-11f32 * mass) / sqr_dist;
                forces.push(force);
            }
        }

        if !p.1.sleeping && !cb.4.sleeping {
            let sqr_dist = (cb.0.translation - p.0.translation).length_squared();
            let force_dir = (cb.0.translation - p.0.translation).normalize_or_zero();
            let force = (force_dir * 6.67430e-11f32 * mass) / sqr_dist;

            // Only one body can affect the player at a time
            if force.length_squared() > c_force.length_squared() {
                let gravity_up = -force.normalize_or_zero();
                let target_rotation = Quat::from_rotation_arc(p.0.up(), gravity_up) * p.0.rotation;

                if !p.3.is_grounded {
                    p.2.linvel += force * time.delta_seconds();
                }
                p.0.rotation = Quat::slerp(p.0.rotation, target_rotation, time.delta_seconds());
                c_force = force;
            }
        }
    }

    for mut cb in cbs_mut.iter_mut() {
        if !cb.4.sleeping {
            cb.5.linvel += forces.pop().unwrap() * time.delta_seconds();
        }
    }
}
