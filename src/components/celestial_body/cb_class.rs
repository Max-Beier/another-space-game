use bevy::prelude::Component;

#[derive(Component, Clone)]
pub enum CBClass {
    Star,
    Planet,
    _Moon,
    _Asteroid,
    _Comet,
    _BlackHole,
    Other,
}

impl Default for CBClass {
    fn default() -> Self {
        CBClass::Other
    }
}
