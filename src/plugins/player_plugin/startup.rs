use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{BuildChildren, Camera3d, Camera3dBundle, Color, Commands, Query, Transform, Vec3},
    transform::TransformBundle,
    window::Window,
};
use bevy_rapier3d::prelude::{
    Ccd, Collider, ColliderMassProperties, GravityScale, RigidBody, Sleeping, Velocity,
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
        .spawn(RigidBody::Dynamic)
        .insert(Sleeping::disabled())
        .insert(Ccd::enabled())
        .insert(GravityScale(1.0))
        .insert(Collider::capsule(Vec3::ZERO, Vec3::new(0.0, 2.0, 0.0), 1.0))
        .insert(ColliderMassProperties::Mass(PlayerBundle::default().mass.0))
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 10000.0, 0.0)),
            ..Default::default()
        })
        .insert(PlayerBundle::default())
        .add_child(camera_e);

    change_cursor(&mut window);
}
