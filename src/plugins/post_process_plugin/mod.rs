use bevy::{
    prelude::{App, AssetServer, Handle, IntoSystemConfigs, Plugin, Shader},
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_resource::SpecializedRenderPipelines,
        renderer::RenderDevice,
        Render, RenderApp, RenderSet,
    },
};

mod pipeline;
mod prepare;
mod queue;

use crate::components::AtmosphereSettings;

use self::pipeline::PostProcessPipeline;

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            ExtractComponentPlugin::<AtmosphereSettings>::default(),
            UniformComponentPlugin::<AtmosphereSettings>::default(),
        ));

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        render_app
            .init_resource::<SpecializedRenderPipelines<PostProcessPipeline>>()
            .add_systems(
                Render,
                (
                    prepare::prepare.in_set(RenderSet::Prepare),
                    queue::queue.in_set(RenderSet::Queue),
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let shader: Handle<Shader> = app
            .world
            .resource::<AssetServer>()
            .load("shaders/post_process.wgsl");

        let render_app = match app.get_sub_app_mut(RenderApp) {
            Ok(render_app) => render_app,
            Err(_) => return,
        };

        let render_device = render_app.world.resource::<RenderDevice>().clone();

        render_app.insert_resource(PostProcessPipeline::new(&render_device, shader));
    }
}
