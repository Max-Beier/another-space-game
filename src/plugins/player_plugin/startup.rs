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

use crate::components::{CBClass, CBRadius, PlayerController};

use super::utils::change_cursor;

pub fn startup(
    mut commands: Commands,
    mut window_q: Query<&mut Window>,
    cbs: Query<(&CBClass, &Transform, &CBRadius)>,
) {
    let mut window = window_q.single_mut();
    change_cursor(&mut window);

    let player_controller = PlayerController::default();
    let player_spawn_positions: Vec<(Vec3, f32)> = cbs
        .iter()
        .filter(|cb| matches!(cb.0, CBClass::Planet))
        .map(|cb| (cb.1.translation, cb.2 .0))
        .collect();
    let player_spawn_postion =
        player_spawn_positions[0].0 + Vec3::new(player_spawn_positions[0].1, 0.0, 0.0) * 1.01;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Collider::capsule_y(1.0, 0.5))
        .insert(ColliderMassProperties::Mass(player_controller.mass))
        .insert(Restitution {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        })
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(player_controller)
        .insert(TransformBundle {
            local: Transform::from_translation(player_spawn_postion),
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
                .spawn(Collider::ball(0.1))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    0.0, -1.0, -1.0,
                )));
        });
}
