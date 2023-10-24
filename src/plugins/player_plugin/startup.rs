use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        BuildChildren, Camera3d, Camera3dBundle, Color, Commands, ComputedVisibility, Query,
        Transform, Vec3,
    },
    transform::TransformBundle,
    window::Window,
};
use bevy_rapier3d::prelude::{
    ActiveEvents, Collider, ColliderMassProperties, RigidBody, Sensor, Velocity,
};

use crate::components::{CBClass, CelestialBody, PlayerController};

use super::utils::change_cursor;

pub fn startup(
    mut commands: Commands,
    mut window_q: Query<&mut Window>,
    cbs: Query<(&CelestialBody, &Transform)>,
) {
    let mut window = window_q.single_mut();
    change_cursor(&mut window);

    let player_controller = PlayerController::default();
    let player_height = player_controller.height;

    let player_spawn_positions: Vec<(Vec3, f32)> = cbs
        .iter()
        .filter(|cb| matches!(cb.0.class, CBClass::Planet))
        .map(|cb| (cb.1.translation, cb.0.radius))
        .collect();

    let player_spawn_postion =
        player_spawn_positions[0].0 + Vec3::new(0.0, 0.0, player_spawn_positions[0].1) * 3.0;

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::capsule_y(
            player_height * 0.5,
            player_height * 0.5,
        ))
        .insert(ColliderMassProperties::Mass(player_controller.mass))
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(player_controller)
        .insert(ComputedVisibility::default())
        .insert(TransformBundle {
            local: Transform::from_translation(player_spawn_postion),
            ..Default::default()
        })
        .with_children(|children| {
            // Camera
            children.spawn((Camera3dBundle {
                camera_3d: Camera3d {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..Default::default()
                },
                ..Default::default()
            },));

            // OnGround-Detection
            children
                .spawn(Collider::ball(0.1))
                .insert(Sensor)
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TransformBundle::from_transform(Transform::from_xyz(
                    0.0,
                    -player_height,
                    0.0,
                )));
        });
}
