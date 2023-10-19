use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        BuildChildren, Camera3d, Camera3dBundle, Color, Commands, Query, Res, Transform, Vec3,
    },
    transform::TransformBundle,
    window::Window,
};
use bevy_rapier3d::prelude::{
    CharacterLength, Collider, ColliderMassProperties, KinematicCharacterController,
    KinematicCharacterControllerOutput, RigidBody, Sleeping, Velocity,
};

use crate::{bundles::PlayerBundle, resources::Player};

use super::utils::change_cursor;

pub fn startup(mut commands: Commands, mut window_q: Query<&mut Window>, player: Res<Player>) {
    let mut window = window_q.single_mut();
    change_cursor(&mut window);

    let player_bundle = PlayerBundle::default();

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
        .spawn(RigidBody::KinematicVelocityBased)
        .insert(Sleeping::disabled())
        .insert(Collider::ball(0.5))
        .insert(ColliderMassProperties::Mass(player_bundle.mass.0))
        .insert(Velocity::linear(Vec3::ZERO))
        .insert(KinematicCharacterController {
            snap_to_ground: Some(CharacterLength::Absolute(0.5)),
            custom_mass: Some(player_bundle.mass.0),
            ..Default::default()
        })
        .insert(KinematicCharacterControllerOutput::default())
        .insert(player_bundle)
        .insert(TransformBundle {
            local: Transform::from_translation(player.spawn_point),
            ..Default::default()
        })
        .add_child(camera_e);
}
