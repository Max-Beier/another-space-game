use bevy::prelude::{App, Plugin, Startup, Update};

mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, systems::startup);
        app.add_systems(Update, systems::update);
    }
}
