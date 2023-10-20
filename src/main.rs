use bevy::{
    pbr::wireframe::WireframePlugin,
    prelude::{App, PluginGroup, PreStartup, ResMut, Vec3},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier3d::prelude::{NoUserData, RapierConfiguration, RapierPhysicsPlugin};
use resources::{InputSettings, Space};

mod bundles;
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
            WireframePlugin,
        ))
        .run();
}

fn preload(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::ZERO;
}
