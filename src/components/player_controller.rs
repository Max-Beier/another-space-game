use bevy::prelude::{Component, Vec3};

pub enum MovementState {
    GROUND,
}

#[derive(Component)]
pub struct PlayerController {
    pub name: String,
    pub spawn_point: Vec3,
    pub ground_speed: f32,
    pub movement_state: MovementState,
    pub mass: f32,
    pub is_grounded: bool,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
            ground_speed: 100.0,
            spawn_point: Vec3::new(10000.0, 10000.0, 10000.0),
            movement_state: MovementState::GROUND,
            mass: 80.0,
            is_grounded: false,
        }
    }
}
