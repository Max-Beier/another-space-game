use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{BuildChildren, Camera3d, Camera3dBundle, Color, Commands, Query, Transform, Vec3},
    transform::TransformBundle,
    window::Window,
};
use bevy_rapier3d::prelude::{
    CharacterLength, Collider, ColliderMassProperties, KinematicCharacterController, RigidBody,
    Sleeping, Velocity,
};

use crate::bundles::PlayerBundle;

use super::utils::change_cursor;

pub fn startup(mut commands: Commands, mut window_q: Query<&mut Window>) {
    let mut window = window_q.single_mut();

    let camera_e = commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(0.5))
        .insert(ColliderMassProperties::Mass(PlayerBundle::default().mass.0))
        .insert(Velocity::default())
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            ..Default::default()
        })
        .insert(TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
        })
        .insert(PlayerBundle::default())
        .add_child(camera_e);

    change_cursor(&mut window);
}
