use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct CBMass(pub f32);

impl Default for CBMass {
    fn default() -> Self {
        Self(10.0)
    }
}
