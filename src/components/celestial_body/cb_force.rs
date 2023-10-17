use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct CBForce(pub Vec3);

impl Default for CBForce {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
