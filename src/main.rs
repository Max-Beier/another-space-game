use bevy::{
    prelude::{App, Commands, PluginGroup, PreStartup, ResMut, Vec3},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use bundles::SettingsBundle;

mod bundles;
mod components;
mod plugins;

fn main() {
    App::new()
        .add_systems(PreStartup, preload)
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Another Space Game...".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            RapierPhysicsPlugin::<NoUserData>::default(),
            plugins::PlayerPlugin,
            plugins::SpacePlugin,
        ))
        .run();
}

fn preload(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    let settings = SettingsBundle::default();
    commands.spawn(settings);

    rapier_config.gravity = Vec3::ZERO;
}
