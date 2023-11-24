use bevy::{
    prelude::{Commands, Component, Entity, Query, Res, With},
    render::{
        extract_component::ComponentUniforms,
        render_resource::{
            BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, SamplerDescriptor,
        },
        renderer::RenderDevice,
        view::{ViewTarget, ViewUniforms},
    },
};

use crate::components::{AtmosphereSettings, CelestialBody};

use super::pipeline::PostProcessPipeline;

#[derive(Component)]
pub struct PostProcessBindGroup(pub BindGroup);

pub fn queue(
    mut commands: Commands,
    pipeline: Res<PostProcessPipeline>,
    view_uniforms: Res<ViewUniforms>,
    atmosphere_uniforms: Res<ComponentUniforms<AtmosphereSettings>>,
    render_device: Res<RenderDevice>,
    entities: Query<Entity, With<CelestialBody>>,
    view_target: Query<&ViewTarget>,
) {
    let post_process = view_target.single().post_process_write();
    let sampler = render_device.create_sampler(&SamplerDescriptor::default());

    for entity in entities.iter() {
        if let (Some(view_uniforms), Some(atmosphere_uniforms)) = (
            view_uniforms.uniforms.binding(),
            atmosphere_uniforms.uniforms().binding(),
        ) {
            let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("post_process_bind_group"),
                layout: &pipeline.bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&post_process.source),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&sampler),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: view_uniforms,
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: atmosphere_uniforms,
                    },
                ],
            });

            commands
                .entity(entity)
                .insert(PostProcessBindGroup(bind_group));
        }
    }
}
