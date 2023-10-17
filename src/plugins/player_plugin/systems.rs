use bevy::{
    prelude::{Commands, Input, KeyCode, Query, Res, Transform, Vec2, Vec3, With},
    time::Time,
    window::{CursorGrabMode, Window},
};

use crate::{bundles::PlayerBundle, components::Player};

fn change_cursor(window: &mut Window) {
    let window_size = Vec2::new(window.width(), window.height());
    window.cursor.grab_mode = match window.cursor.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
    };
    window.set_cursor_position(Some(Vec2::new(window_size.x * 0.5, window_size.y * 0.5)));
    window.cursor.visible = !window.cursor.visible;
}

pub fn startup(mut commands: Commands, mut window_q: Query<&mut Window>) {
    let mut window = window_q.single_mut();
    change_cursor(&mut window);

    let player = PlayerBundle::default();
    commands.spawn(player);
}

pub fn update(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Player), With<Player>>,
    mut window_q: Query<&mut Window>,
) {
    let (mut transform, player) = player_q.single_mut();
    let mut window = window_q.single_mut();
    let mut direction: Vec3 = Vec3::ZERO;

    if input.pressed(KeyCode::W) {
        direction += transform.forward();
    }

    if input.pressed(KeyCode::A) {
        direction += transform.left();
    }

    if input.pressed(KeyCode::S) {
        direction += transform.back();
    }

    if input.pressed(KeyCode::D) {
        direction += transform.right();
    }

    if input.just_pressed(KeyCode::Escape) {
        change_cursor(&mut window);
    }

    let movement = direction.normalize_or_zero() * player.speed * time.delta_seconds();
    transform.translation += movement;
}
