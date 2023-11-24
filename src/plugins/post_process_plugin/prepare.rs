use bevy::{
    prelude::{Commands, Entity, Query, Res, ResMut, With},
    render::render_resource::{PipelineCache, SpecializedRenderPipelines},
};

use crate::components::CelestialBody;

use super::pipeline::{PostProcessPipeline, PostProcessPipelineId};

pub fn prepare(
    mut commands: Commands,
    pipeline_cache: Res<PipelineCache>,
    mut pipelines: ResMut<SpecializedRenderPipelines<PostProcessPipeline>>,
    pipeline: Res<PostProcessPipeline>,
    entities: Query<Entity, With<CelestialBody>>,
) {
    for entity in entities.iter() {
        let pipeline_id = pipelines.specialize(&pipeline_cache, &pipeline, ());

        commands
            .entity(entity)
            .insert(PostProcessPipelineId(pipeline_id));
    }
}
