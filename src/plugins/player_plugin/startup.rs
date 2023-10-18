use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{BuildChildren, Camera3d, Camera3dBundle, Color, Commands, Query},
    transform::TransformBundle,
    window::Window,
};

use crate::bundles::PlayerBundle;

use super::utils::change_cursor;

pub fn startup(mut commands: Commands, mut window_q: Query<&mut Window>) {
    let mut window = window_q.single_mut();

    let camera_e = commands
        .spawn(Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::BLACK),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .spawn(PlayerBundle::default())
        .insert(TransformBundle {
            ..Default::default()
        })
        .add_child(camera_e);

    change_cursor(&mut window);
}
