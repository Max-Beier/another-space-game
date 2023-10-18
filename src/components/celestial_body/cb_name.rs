use bevy::prelude::Component;

#[derive(Component, Clone)]
pub struct CBName(pub String);

impl Default for CBName {
    fn default() -> Self {
        Self("Unknown Body".to_string())
    }
}
