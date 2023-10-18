use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct CBSpin(pub f32);

impl Default for CBSpin {
    fn default() -> Self {
        Self(10.0)
    }
}
