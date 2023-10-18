use bevy::prelude::Bundle;

use crate::components::PName;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PName,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
        }
    }
}
