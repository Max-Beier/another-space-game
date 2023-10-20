use bevy::{
    input::mouse::MouseMotion,
    prelude::{
        Camera3d, EulerRot, EventReader, Input, KeyCode, Quat, Query, Res, Transform, Vec3, With,
        Without,
    },
    time::Time,
    window::{CursorGrabMode, Window},
};
use bevy_rapier3d::prelude::{CollisionEvent, Velocity};

use crate::{components::PlayerController, resources::InputSettings};

use super::utils::change_cursor;

mod foot;

pub fn update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    input_settings: Res<InputSettings>,
    mut collision_events: EventReader<CollisionEvent>,
    mut mouse_event: EventReader<MouseMotion>,
    mut window_query: Query<&mut Window>,
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &mut PlayerController),
        With<PlayerController>,
    >,
    mut camera_query: Query<&mut Transform, (With<Camera3d>, Without<PlayerController>)>,
) {
    let mut window = window_query.single_mut();
    let (mut player_transform, mut player_velocity, mut player_controller) =
        player_query.single_mut();
    let mut camera_transform = camera_query.single_mut();

    // Pause
    if input.just_pressed(KeyCode::Escape) {
        change_cursor(&mut window);
    }

    for _ in collision_events.iter() {
        player_controller.is_colliding = !player_controller.is_colliding;
        println!("On Ground: {}", player_controller.is_colliding);
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

        foot::active(
            &input,
            &time,
            &mut player_transform,
            &mut player_velocity,
            &player_controller,
        )
    };
}
