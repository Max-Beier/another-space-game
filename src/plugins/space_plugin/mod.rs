use bevy::prelude::{App, Plugin, Startup, Update};

mod generate;
mod startup;
mod update;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup::startup);
        app.add_systems(Update, update::update);
    }
}
