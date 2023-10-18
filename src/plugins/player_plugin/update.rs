use bevy::{
    input::mouse::MouseMotion,
    prelude::{
        Camera3d, EulerRot, EventReader, Input, KeyCode, Quat, Query, Res, Transform, Vec3, With,
        Without,
    },
    time::Time,
    window::{CursorGrabMode, Window},
};

use crate::{
    components::{PMass, PName},
    resources::{InputSettings, Player},
};

use super::utils::change_cursor;

pub fn update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    input_settings: Res<InputSettings>,
    player: Res<Player>,
    mut mouse_event: EventReader<MouseMotion>,
    mut window_q: Query<&mut Window>,
    mut player_q: Query<&mut Transform, (With<PName>, With<PMass>, Without<Camera3d>)>,
    mut camera_q: Query<&mut Transform, (With<Camera3d>, Without<PName>)>,
) {
    let mut window: bevy::prelude::Mut<'_, Window> = window_q.single_mut();
    let mut player_tranform = player_q.single_mut();
    let mut camera_transform = camera_q.single_mut();
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

    if input.pressed(KeyCode::Space) {
        direction += player_tranform.up();
    }

    if input.pressed(KeyCode::ControlLeft) {
        direction += player_tranform.down();
    }

    if input.just_pressed(KeyCode::Escape) {
        change_cursor(&mut window);
    }

    if window.cursor.grab_mode == CursorGrabMode::Locked {
        for event in mouse_event.iter() {
            let mut camera_pitch = camera_transform.rotation.to_euler(EulerRot::XYZ).0;
            camera_pitch -= event.delta.y * input_settings.mouse_sensitivity * 0.001;
            camera_pitch = camera_pitch
                .max(-89.9_f32.to_radians())
                .min(89.0_f32.to_radians());
            camera_transform.rotation = Quat::from_euler(EulerRot::XYZ, camera_pitch, 0.0, 0.0);

            player_tranform.rotate_axis(
                Vec3::NEG_Y,
                event.delta.x * input_settings.mouse_sensitivity * 0.001,
            );
        }
    };

    let movement = direction.normalize_or_zero() * player.speed * time.delta_seconds();
    player_tranform.translation += movement;
}
