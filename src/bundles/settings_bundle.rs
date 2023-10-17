use bevy::prelude::Bundle;

use crate::components::InputSettings;

#[derive(Bundle)]
pub struct SettingsBundle {
    input_settings: InputSettings,
}

impl Default for SettingsBundle {
    fn default() -> Self {
        Self {
            input_settings: InputSettings::default(),
        }
    }
}
