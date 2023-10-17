use bevy::{
    prelude::{App, Commands, PluginGroup, PreStartup, Query},
    render::view::window,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
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
            plugins::PlayerPlugin,
            plugins::SpacePlugin,
        ))
        .run();
}

fn preload(mut commands: Commands) {
    let settings = SettingsBundle::default();
    commands.spawn(settings);
}
