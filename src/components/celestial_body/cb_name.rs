use bevy::prelude::Component;

#[derive(Component)]
pub struct CBName(pub String);

impl Default for CBName {
    fn default() -> Self {
        Self("Unknown Body".to_string())
    }
}
