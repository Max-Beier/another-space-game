use bevy::prelude::{Component, Vec3};

#[derive(Component)]
pub struct CBVelocity(pub Vec3);

impl Default for CBVelocity {
    fn default() -> Self {
        Self(Vec3::ZERO)
    }
}
