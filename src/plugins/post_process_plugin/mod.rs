use bevy::{
    core_pipeline::core_3d,
    prelude::{App, Plugin},
    render::{
        extract_component::{ExtractComponentPlugin, UniformComponentPlugin},
        render_graph::{RenderGraphApp, ViewNodeRunner},
        RenderApp,
    },
};

mod post_process_node;

pub use post_process_node::*;

use crate::{
    components::{AtmosphereSettings, View},
    resources::PostProcessPipeline,
};

pub struct PostProcessPlugin;

impl Plugin for PostProcessPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            ExtractComponentPlugin::<View>::default(),
            UniformComponentPlugin::<View>::default(),
            ExtractComponentPlugin::<AtmosphereSettings>::default(),
            UniformComponentPlugin::<AtmosphereSettings>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app
            .add_render_graph_node::<ViewNodeRunner<PostProcessNode>>(
                core_3d::graph::NAME,
                PostProcessNode::NAME,
            )
            .add_render_graph_edges(
                core_3d::graph::NAME,
                &[
                    core_3d::graph::node::TONEMAPPING,
                    PostProcessNode::NAME,
                    core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
                ],
            );
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<PostProcessPipeline>();
    }
}
