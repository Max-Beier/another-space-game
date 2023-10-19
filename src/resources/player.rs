use bevy::prelude::{Resource, Vec3};

#[derive(Resource)]
pub struct Player {
    pub spawn_point: Vec3,
    pub speed: f32,
    pub jump_velocity: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            spawn_point: Vec3::new(1000.0, 0.0, 0.0),
            speed: 1000.0,
            jump_velocity: 1000.0,
        }
    }
}
