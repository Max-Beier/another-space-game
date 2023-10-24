use bevy::{
    ecs::query::QueryItem,
    prelude::World,
    render::{
        extract_component::ComponentUniforms,
        render_graph::{NodeRunError, RenderGraphContext, ViewNode},
        render_resource::{
            BindGroupDescriptor, BindGroupEntry, BindingResource, Operations, PipelineCache,
            RenderPassColorAttachment, RenderPassDescriptor,
        },
        renderer::RenderContext,
        view::{ViewTarget, ViewUniformOffset, ViewUniforms},
    },
};

use crate::{components::AtmosphereSettings, resources::PostProcessPipeline};

#[derive(Default)]
pub struct PostProcessNode;
impl PostProcessNode {
    pub const NAME: &'static str = "post_process";
}

impl ViewNode for PostProcessNode {
    type ViewQuery = (&'static ViewTarget, &'static ViewUniformOffset);

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        view_target: QueryItem<Self::ViewQuery>,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<PostProcessPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let atmosphere_settings_uniform = world.resource::<ComponentUniforms<AtmosphereSettings>>();
        let Some(atmosphere_settings_binding) = atmosphere_settings_uniform.uniforms().binding() else {
            return Ok(());
        };

        let view_uniforms = world.resource::<ViewUniforms>();
        let Some(view_uniforms_binding) = view_uniforms.uniforms.binding() else {
            return Ok(());
        };

        let post_process = view_target.0.post_process_write();

        let bind_group = render_context
            .render_device()
            .create_bind_group(&BindGroupDescriptor {
                label: Some("post_process_bind_group"),
                layout: &post_process_pipeline.layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(post_process.source),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&post_process_pipeline.sampler),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: atmosphere_settings_binding.clone(),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: view_uniforms_binding.clone(),
                    },
                ],
            });

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
            })],
            depth_stencil_attachment: None,
        });

        let offset = view_target.1.offset;
        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[offset]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}
