use bevy::{
    prelude::{Input, KeyCode, Res, Transform, Vec3},
    time::Time,
};
use bevy_rapier3d::prelude::Velocity;

use crate::components::PlayerController;

pub fn active(
    input: &Res<Input<KeyCode>>,
    time: &Res<Time>,
    player_transform: &mut Transform,
    player_velocity: &mut Velocity,
    player_controller: &PlayerController,
) {
    let mut direction: Vec3 = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        direction += player_transform.forward();
    }

    if input.pressed(KeyCode::S) {
        direction -= player_transform.forward();
    }

    if input.pressed(KeyCode::A) {
        direction -= player_transform.right();
    }

    if input.pressed(KeyCode::D) {
        direction += player_transform.right();
    }

    if input.pressed(KeyCode::Space) {
        direction += player_transform.up();
    }

    if input.pressed(KeyCode::ControlLeft) {
        direction += player_transform.down();
    }

    player_velocity.linvel +=
        direction.normalize_or_zero() * player_controller.ground_speed * time.delta_seconds();
}
