use bevy::prelude::Component;

#[derive(Component)]
pub struct InputSettings {
    pub mouse_sensitivity: f32,
}

impl Default for InputSettings {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 1.0,
        }
    }
}
