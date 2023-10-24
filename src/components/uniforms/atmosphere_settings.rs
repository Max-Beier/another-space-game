use bevy::{
    prelude::{Component, Vec3},
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct AtmosphereSettings {
    pub center: Vec3,
    pub ground_radius: f32,
    pub atmosphere_radius: f32,
    pub light_dir: Vec3,
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            ground_radius: 100000.0,
            atmosphere_radius: 13000.0,
            light_dir: Vec3::ZERO,
        }
    }
}
