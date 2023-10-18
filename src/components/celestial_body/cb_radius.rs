use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct CBRadius(pub f32);

impl Default for CBRadius {
    fn default() -> Self {
        Self(10.0)
    }
}
