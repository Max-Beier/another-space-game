use bevy::prelude::{Bundle, ComputedVisibility, GlobalTransform, Transform, Visibility};

use crate::components::{CBClass, CBForce, CBName, CBRadius, CBVelocity};

#[derive(Bundle)]
pub struct CBBundle {
    pub name: CBName,
    pub class: CBClass,
    pub radius: CBRadius,
    pub velocity: CBVelocity,
    pub force: CBForce,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
}

impl Default for CBBundle {
    fn default() -> Self {
        Self {
            name: Default::default(),
            class: Default::default(),
            radius: Default::default(),
            velocity: Default::default(),
            force: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            visibility: Default::default(),
            computed_visibility: Default::default(),
        }
    }
}
