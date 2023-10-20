use bevy::{
    input::mouse::MouseMotion,
    prelude::{
        Camera3d, EulerRot, EventReader, Input, KeyCode, Quat, Query, Res, Transform, Vec3, With,
        Without,
    },
    time::Time,
    window::{CursorGrabMode, Window},
};
use bevy_rapier3d::prelude::Velocity;

use crate::{
    components::{MovementState, PlayerController},
    resources::InputSettings,
};

use super::utils::change_cursor;

mod ground;
mod space;

pub fn update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    input_settings: Res<InputSettings>,
    mut mouse_event: EventReader<MouseMotion>,
    mut window_query: Query<&mut Window>,
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &PlayerController),
        With<PlayerController>,
    >,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PlayerController>)>,
) {
    let mut window = window_query.single_mut();
    let (mut player_transform, mut player_velocity, player_controller) = player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    if input.just_pressed(KeyCode::Escape) {
        change_cursor(&mut window);
    }

    if window.cursor.grab_mode == CursorGrabMode::Locked {
        for event in mouse_event.iter() {
            // Camera Pitch (UP/DOWN)
            let mut camera_pitch = camera_transform.rotation.to_euler(EulerRot::XYZ).0;
            camera_pitch -= event.delta.y * input_settings.mouse_sensitivity * 0.001;
            camera_pitch = camera_pitch
                .max(-89.9_f32.to_radians())
                .min(89.9_f32.to_radians());
            camera_transform.rotation = Quat::from_euler(EulerRot::XYZ, camera_pitch, 0.0, 0.0);

            // Player Yaw (LEFT/RIGHT)
            let mut player_yar = camera_transform.rotation.to_euler(EulerRot::XYZ).1;
            player_yar -= event.delta.x * input_settings.mouse_sensitivity * 0.001;
            player_transform.rotate_local_axis(Vec3::Y, player_yar);
        }
    };

    match player_controller.movement_state {
        MovementState::GROUND => ground::active(
            &input,
            &time,
            &mut player_transform,
            &mut player_velocity,
            &player_controller,
        ),
    }
}
