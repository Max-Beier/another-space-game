use bevy::prelude::{App, Plugin, Startup, Update};

mod movement;
mod startup;
mod utils;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup::startup);
        app.add_systems(Update, movement::update);
    }
}
