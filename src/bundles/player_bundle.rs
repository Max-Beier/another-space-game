use bevy::prelude::Bundle;

use crate::components::{PMass, PName};

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: PName,
    pub mass: PMass,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            mass: Default::default(),
        }
    }
}
