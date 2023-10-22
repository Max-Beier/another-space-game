use bevy::{
    prelude::Component,
    render::{extract_component::ExtractComponent, render_resource::ShaderType},
};

#[derive(Component, Clone, Copy, ExtractComponent, ShaderType)]
pub struct AtmosphereSettings {
    pub extent: f32,
}

impl Default for AtmosphereSettings {
    fn default() -> Self {
        Self { extent: 100.0 }
    }
}
