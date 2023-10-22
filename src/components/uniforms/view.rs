use bevy::{
    prelude::{Component, Vec3},
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct View {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Default for View {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            direction: Vec3::ZERO,
        }
    }
}
