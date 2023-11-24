use bevy::{
    core_pipeline::fullscreen_vertex_shader::fullscreen_shader_vertex_state,
    prelude::{Component, Handle, Resource, Shader},
    render::{
        render_resource::{
            BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType,
            BufferBindingType, CachedRenderPipelineId, ColorTargetState, ColorWrites,
            CompareFunction, DepthBiasState, DepthStencilState, FragmentState, MultisampleState,
            PrimitiveState, RenderPipelineDescriptor, SamplerBindingType, ShaderStages, ShaderType,
            SpecializedRenderPipeline, StencilFaceState, StencilState, TextureFormat,
            TextureSampleType, TextureViewDimension,
        },
        renderer::RenderDevice,
        texture::BevyDefault,
        view::ViewUniform,
    },
};

use crate::components::AtmosphereSettings;

#[derive(Resource)]
pub struct PostProcessPipeline {
    pub bind_group_layout: BindGroupLayout,
    pub shader: Handle<Shader>,
}

#[derive(Component)]
pub struct PostProcessPipelineId(pub CachedRenderPipelineId);

// https://github.com/bevyengine/bevy/blob/60773e6787d177e97458f9fcf118985906762b2a/examples/shader/post_processing.rs#L217C7-L217C7
impl PostProcessPipeline {
    pub fn new(render_device: &RenderDevice, shader: Handle<Shader>) -> Self {
        let bind_group_layout_descriptor = BindGroupLayoutDescriptor {
            label: Some("post_process_bind_group_layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(ViewUniform::min_size()),
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 3,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: bevy::render::render_resource::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: Some(AtmosphereSettings::min_size()),
                    },
                    count: None,
                },
            ],
        };

        Self {
            bind_group_layout: render_device
                .create_bind_group_layout(&bind_group_layout_descriptor),
            shader,
        }
    }
}

impl SpecializedRenderPipeline for PostProcessPipeline {
    type Key = ();
    fn specialize(&self, _key: Self::Key) -> RenderPipelineDescriptor {
        let shader = self.shader.clone();
        RenderPipelineDescriptor {
            label: Some("post_process_pipeline".into()),
            layout: vec![self.bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            vertex: fullscreen_shader_vertex_state(),
            primitive: PrimitiveState::default(),
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: false,
                depth_compare: CompareFunction::GreaterEqual,
                stencil: StencilState {
                    front: StencilFaceState::IGNORE,
                    back: StencilFaceState::IGNORE,
                    read_mask: 0,
                    write_mask: 0,
                },
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0,
                },
            }),
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                shader: shader,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
        }
    }
}
