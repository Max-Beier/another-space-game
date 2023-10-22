use std::time::Duration;

use bevy::{
    asset::ChangeWatcher,
    prelude::{App, AssetPlugin, PluginGroup, PreStartup, ResMut, Vec3},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use resources::{InputSettings, Space};

mod components;
mod plugins;
mod resources;

fn main() {
    App::new()
        // Resources
        .insert_resource(InputSettings::default())
        .insert_resource(Space::default())
        // Systems
        .add_systems(PreStartup, preload)
        // Plugins
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Another Space Game...".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(AssetPlugin {
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..Default::default()
                }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            plugins::PlayerPlugin,
            plugins::SpacePlugin,
            plugins::PostProcessPlugin,
        ))
        .run();
}

fn preload(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::ZERO;
}
