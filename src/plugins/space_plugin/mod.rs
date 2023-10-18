use bevy::prelude::{App, Plugin, Startup};

mod generate;
mod startup;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup::startup);
    }
}
