use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::{
        Assets, BuildChildren, Camera3d, Camera3dBundle, Color, Commands, Mesh, Query, ResMut,
        StandardMaterial,
    },
    transform::TransformBundle,
    window::Window,
};

use crate::bundles::PlayerBundle;

use super::utils::change_cursor;

pub fn startup(
    mut commands: Commands,
    mut window_q: Query<&mut Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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
        .insert(TransformBundle::default())
        .add_child(camera_e);

    change_cursor(&mut window);
}
