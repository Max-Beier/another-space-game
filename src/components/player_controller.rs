use bevy::prelude::Component;

#[derive(Component)]
pub struct PlayerController {
    pub name: String,
    pub speed: f32,
    pub jump: f32,
    pub mass: f32,
    pub is_colliding: bool,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            name: "Player".to_string(),
            speed: 100.0,
            jump: 100.0,
            mass: 80.0,
            is_colliding: false,
        }
    }
}
