use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    input::mouse::MouseMotion,
    prelude::{
        BuildChildren, Camera3d, Camera3dBundle, Color, Commands, EulerRot, EventReader, Input,
        KeyCode, Quat, Query, Res, Transform, Vec2, Vec3, With, Without,
    },
    time::Time,
    window::{CursorGrabMode, Window},
};

use crate::{
    bundles::PlayerBundle,
    components::{InputSettings, PName},
};

// CONSTANTS
const PLAYER_SPEED: f32 = 10.0;

pub fn startup(mut commands: Commands, mut window_q: Query<&mut Window>) {
    let mut window = window_q.single_mut();

    let player = PlayerBundle::default();
    let camera = Camera3dBundle {
        camera_3d: Camera3d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
            ..Default::default()
        },
        ..Default::default()
    };

    let camera_entity = commands.spawn(camera).id();
    commands.spawn(player).add_child(camera_entity);

    change_cursor(&mut window);
}

pub fn update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut mouse_event: EventReader<MouseMotion>,
    mut window_q: Query<&mut Window>,
    mut camera_q: Query<&mut Transform, (With<Camera3d>, Without<PName>)>,
    mut player_q: Query<&mut Transform, With<PName>>,
    input_settings_q: Query<&InputSettings>,
) {
    let input_settings = input_settings_q.single();
    let mut camera_transform = camera_q.single_mut();
    let mut player_tranform = player_q.single_mut();
    let mut window: bevy::prelude::Mut<'_, Window> = window_q.single_mut();
    let mut direction: Vec3 = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        direction += player_tranform.forward();
    }

    if input.pressed(KeyCode::A) {
        direction += player_tranform.left();
    }

    if input.pressed(KeyCode::S) {
        direction += player_tranform.back();
    }

    if input.pressed(KeyCode::D) {
        direction += player_tranform.right();
    }

    if input.just_pressed(KeyCode::Escape) {
        change_cursor(&mut window);
    }

    if window.cursor.grab_mode == CursorGrabMode::Locked {
        for event in mouse_event.iter() {
            let (mut camera_pitch, camera_yaw, camera_roll) =
                camera_transform.rotation.to_euler(EulerRot::XYZ);
            camera_pitch -= event.delta.y * input_settings.mouse_sensitivity * 0.001;
            camera_transform.rotation =
                Quat::from_euler(EulerRot::XYZ, camera_pitch, camera_yaw, camera_roll);

            let (pbr_pitch, mut pbr_yaw, pbr_roll) =
                player_tranform.rotation.to_euler(EulerRot::XYZ);
            pbr_yaw -= event.delta.x * input_settings.mouse_sensitivity * 0.001;
            player_tranform.rotation = Quat::from_euler(EulerRot::XYZ, pbr_pitch, pbr_yaw, pbr_roll)
        }
    };

    let movement = direction.normalize_or_zero() * PLAYER_SPEED * time.delta_seconds();
    player_tranform.translation += movement;
}

fn change_cursor(window: &mut Window) {
    let window_size = Vec2::new(window.width(), window.height());
    window.cursor.grab_mode = match window.cursor.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
    };
    window.set_cursor_position(Some(Vec2::new(window_size.x * 0.5, window_size.y * 0.5)));
    window.cursor.visible = !window.cursor.visible;
}
