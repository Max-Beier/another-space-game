use bevy::prelude::Component;

#[derive(Component)]
pub struct PName(pub String);

impl Default for PName {
    fn default() -> Self {
        Self("Player".to_string())
    }
}
