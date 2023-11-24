use bevy::{
    app::Update,
    prelude::{App, Plugin},
};

mod components;
mod update;

pub use components::*;

pub struct VoxelPlugin;

impl Plugin for VoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update::update);
    }
}
