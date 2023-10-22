use bevy::{
    prelude::{Input, KeyCode, Res, Transform},
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
    if input.pressed(KeyCode::W) {
        player_velocity.linvel +=
            player_transform.forward() * player_controller.velocity * time.delta_seconds();
    }

    if input.pressed(KeyCode::S) {
        player_velocity.linvel +=
            player_transform.back() * player_controller.velocity * time.delta_seconds();
    }

    if input.pressed(KeyCode::A) {
        player_velocity.linvel +=
            player_transform.left() * player_controller.velocity * time.delta_seconds();
    }

    if input.pressed(KeyCode::D) {
        player_velocity.linvel +=
            player_transform.right() * player_controller.velocity * time.delta_seconds();
    }

    if input.pressed(KeyCode::Space) {
        player_velocity.linvel +=
            player_transform.up() * player_controller.velocity * time.delta_seconds();
    }

    if input.pressed(KeyCode::ControlLeft) {
        player_velocity.linvel +=
            player_transform.down() * player_controller.velocity * time.delta_seconds();
    }
}
