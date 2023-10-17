use bevy::prelude::Component;

#[derive(Component)]
pub enum CBClass {
    Star,
    Planet,
    Moon,
    Asteroid,
    Comet,
    BlackHole,
    Other,
}

impl Default for CBClass {
    fn default() -> Self {
        CBClass::Other
    }
}
