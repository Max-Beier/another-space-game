use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerController {
    pub name: String,
    pub height: f32,
    pub velocity: f32,
    pub jump: f32,
    pub mass: f32,
    pub is_colliding: bool,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
            height: 2.0,
            velocity: 1000.0,
            jump: 1000.0,
            mass: 80.0,
            is_colliding: false,
        }
    }
}
