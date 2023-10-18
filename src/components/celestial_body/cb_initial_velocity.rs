use bevy::prelude::{Component, Vec3};

#[derive(Component, Clone)]
pub struct CBInitialVelocity(pub Vec3);

impl Default for CBInitialVelocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
