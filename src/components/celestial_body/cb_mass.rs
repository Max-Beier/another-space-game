use bevy::prelude::Component;

#[derive(Component)]
pub struct CBMass(pub f32);

impl Default for CBMass {
    fn default() -> Self {
        Self(5.0)
    }
}
