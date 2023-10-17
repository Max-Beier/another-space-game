use bevy::{prelude::App, DefaultPlugins};

mod bundles;
mod components;
mod plugins;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, plugins::PlayerPlugin, plugins::SpacePlugin))
        .run();
}
