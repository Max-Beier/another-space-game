#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

struct View {
    origin: vec3<f32>,
    direction: vec3<f32>,
}

struct AtmosphereSettings {
    origin: vec3<f32>,
    ground: f32,
    thickness: f32,
    light_direction: vec3<f32>,
}

@group(0) @binding(0)
var screen_texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;
@group(0) @binding(2)
var<uniform> view: View;
@group(0) @binding(3)
var<uniform> atmosphere: AtmosphereSettings;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let color = vec3<f32>(textureSample(screen_texture, texture_sampler, in.uv).r, textureSample(screen_texture, texture_sampler, in.uv).g, textureSample(screen_texture, texture_sampler, in.uv).b);

    let ray_origin = view.origin;
    let ray_dir = normalize(view.direction);

    let thickness = atmosphere.ground + atmosphere.thickness;

    let hit_info = ray_sphere(atmosphere.origin, thickness, ray_origin, ray_dir);
    let dst_to_atmosphere = hit_info.x;
    let dst_through_atmosphere = hit_info.y;

    let res = dst_through_atmosphere / (thickness * 2.0);

    return vec4<f32>(
        color + res,
        1.0
    );
}

// HELPERS
fn ray_sphere(sphere_center: vec3<f32>, sphere_radius: f32, ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec2<f32> {
    let oc = ray_origin - sphere_center;
    let a = dot(ray_direction, ray_direction);
    let b = 2.0 * dot(oc, ray_direction);
    let c = dot(oc, oc) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;
   
    if (discriminant > 0.0) {
        let s = sqrt(discriminant);
        let dst_to_sphere_near = max(0.0, (-b - s) / (2.0 * a));
        let dst_to_sphere_far = (-b + s) / (2.0 * a);


        if (dst_to_sphere_far >= 0.0) {
            return vec2<f32>(dst_to_sphere_near, dst_to_sphere_far);
        }
    }

    return vec2<f32>(999999.0, 0.0);
};