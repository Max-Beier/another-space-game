use bevy::prelude::Bundle;

use crate::components::{CBClass, CBInitialVelocity, CBName, CBRadius, CBSpin, CBSurfaceGravity};

#[derive(Bundle, Clone)]
pub struct CBBundle {
    pub name: CBName,
    pub class: CBClass,
    pub radius: CBRadius,
    pub surface_gravity: CBSurfaceGravity,
    pub initial_velocity: CBInitialVelocity,
    pub spin: CBSpin,
}

impl Default for CBBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            class: Default::default(),
            radius: Default::default(),
            surface_gravity: Default::default(),
            initial_velocity: Default::default(),
            spin: Default::default(),
        }
    }
}
