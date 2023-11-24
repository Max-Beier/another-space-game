use bevy::{
    prelude::{Component, Vec3},
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct AtmosphereSettings {
    pub origin: Vec3,
    pub ground: f32,
    pub thickness: f32,
    pub light_dir: Vec3,
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            ground: 6371008.767,
            thickness: 16000.0,
            light_dir: Vec3::ZERO,
        }
    }
}
