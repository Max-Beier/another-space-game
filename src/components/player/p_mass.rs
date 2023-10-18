use bevy::prelude::Component;

#[derive(Component)]
pub struct PMass(pub f32);

impl Default for PMass {
    fn default() -> Self {
        Self(80.0)
    }
}
