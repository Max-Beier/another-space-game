use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{BuildChildren, Camera3d, Camera3dBundle, Color, Commands, Query, Transform, Vec3},
    transform::TransformBundle,
    window::Window,
};
use bevy_rapier3d::prelude::{
    ActiveEvents, CoefficientCombineRule, Collider, ColliderMassProperties, Restitution, RigidBody,
    Sensor, Sleeping, Velocity,
};

use crate::components::PlayerController;

use super::utils::change_cursor;

pub fn startup(mut commands: Commands, mut window_q: Query<&mut Window>) {
    let mut window = window_q.single_mut();
    change_cursor(&mut window);

    let player_controller = PlayerController::default();
    let spawn_point = player_controller.spawn_point;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(1.0))
        .insert(ColliderMassProperties::Mass(player_controller.mass))
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(player_controller)
        .insert(TransformBundle {
            local: Transform::from_translation(spawn_point),
            ..Default::default()
        })
        .with_children(|children| {
            children.spawn(Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..Default::default()
                },
                ..Default::default()
            });
            children
                .spawn(Collider::cylinder(1.0, 0.5))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    0.0, -1.0, -1.0,
                )));
        });
}
