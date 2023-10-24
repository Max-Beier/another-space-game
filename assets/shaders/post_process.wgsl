#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
struct PostProcessSettings {
    atmosphere_extent: f32,
}
@group(0) @binding(2)
var<uniform> settings: PostProcessSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = vec4<f32>(
        textureSample(screen_texture, texture_sampler, in.uv).rgb,
        1.0
    );
    return color;
}