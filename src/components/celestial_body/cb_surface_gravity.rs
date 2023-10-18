use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct CBSurfaceGravity(pub f32);

impl Default for CBSurfaceGravity {
    fn default() -> Self {
        Self(9.81)
    }
}
