use bevy::prelude::{App, Plugin, PreStartup, Update};

mod generate;
mod startup;
mod update;

pub struct SpacePlugin;

impl Plugin for SpacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, startup::startup);
        app.add_systems(Update, (update::update, update::foating_point_origin));
    }
}
