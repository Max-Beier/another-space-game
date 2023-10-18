use bevy::{
    prelude::Vec2,
    window::{CursorGrabMode, Window},
};

pub fn change_cursor(window: &mut Window) {
    let window_size = Vec2::new(window.width(), window.height());
    window.cursor.grab_mode = match window.cursor.grab_mode {
        CursorGrabMode::None => CursorGrabMode::Locked,
        CursorGrabMode::Locked | CursorGrabMode::Confined => CursorGrabMode::None,
    };
    window.set_cursor_position(Some(Vec2::new(window_size.x * 0.5, window_size.y * 0.5)));
    window.cursor.visible = !window.cursor.visible;
}
