use bevy::prelude::{Plugin, Startup};

mod atmosphere;

pub struct PostProcessingPlugin;

impl Plugin for PostProcessingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, atmosphere::startup);
    }
}
