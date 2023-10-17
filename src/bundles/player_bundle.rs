use bevy::prelude::{Bundle, Camera3dBundle};

use crate::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    camera_bundle: Camera3dBundle,
    player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            camera_bundle: Camera3dBundle::default(),
            player: Player::default(),
        }
    }
}
