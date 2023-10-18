use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 10000.0 }
    }
}
