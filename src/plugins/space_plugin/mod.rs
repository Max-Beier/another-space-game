use bevy::prelude::{App, Plugin, Startup};

mod systems;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::startup);
    }
}
