#import bevy_render::view View
#import bevy_core_pipeline::fullscreen_vertex_shader FullscreenVertexOutput

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
    let color = vec4<f32>(textureSample(screen_texture, texture_sampler, in.uv).rgb, 1.0);

    let ray_origin = view.inverse_projection[3].xyz;
    let ray_direction = normalize(in.position.xyz - ray_origin);

    let hit_info = ray_sphere(atmosphere.origin, atmosphere.thickness + atmosphere.ground, ray_origin, ray_direction);
    let dst_to_atmosphere = hit_info.x;
    let dst_through_atmosphere = hit_info.y;

    return color * -1.0;
}

// HELPERS
fn ray_sphere(sphere_center: vec3<f32>, sphere_radius: f32, ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> vec2<f32> {
    let oc = ray_origin - sphere_center;
    let a = 1.0;
    let b = 2.0 * dot(oc, ray_direction);
    let c = dot(oc, oc) - sphere_radius * sphere_radius;
    let d = b * b - 4.0 * a * c;

    if (d > 0.0) {
        let s = sqrt(d);
        let d1 = max(0.0, (-b - s) / (2.0 * a));
        let d2 = (-b + s) / (2.0 * a);

        if (d1 > 0.0) {
            return vec2<f32>(d1, d2 - d1);
        }
    }

    return vec2<f32>(99999999999999999999.9, 0.0);
};

fn density_at_point(density_sample_point: vec3<f32>) -> f32 {
    let falloff = 13.0;
    let height_above_surface = length(density_sample_point - atmosphere.origin) - atmosphere.ground;
    let height01 = height_above_surface / atmosphere.thickness;
    let local_density = exp(-height01 * height01) * (1.0 - falloff);
    return local_density;
}

fn optical_depth(ray_origin: vec3<f32>, ray_direction: vec3<f32>, ray_length: f32) -> f32 {
    let num_optical_depth_points = 10;
    var in_scatter_point = ray_origin;
    let step_size = ray_length / f32(num_optical_depth_points - 1);
    var optical_depth = 0.0;
    
    for (var i: i32 = 0; i < num_optical_depth_points; i = i + 1) {
        let local_density = density_at_point(in_scatter_point);
        optical_depth += local_density * step_size;
        in_scatter_point += ray_direction * step_size; // corrected ray_direction variable
    }

    return optical_depth;
}

fn calculate_light(ray_origin: vec3<f32>, ray_direction: vec3<f32>, ray_length: f32) -> vec3<f32> {
    let num_in_scattering_points = 10;
    var in_scatter_point = ray_origin;
    let step_size = ray_length / f32(num_in_scattering_points - 1);
    var in_scattered_light = vec3<f32>(0.0, 0.0, 0.0);
    for (var i: i32 = 0; i < num_in_scattering_points; i = i + 1) {
        let sun_ray_length = ray_sphere(atmosphere.origin, atmosphere.thickness + atmosphere.ground, in_scatter_point, atmosphere.light_direction).y;
        let sun_ray_optical_depth = optical_depth(in_scatter_point, atmosphere.light_direction, sun_ray_length);
        let view_ray_optical_depth = optical_depth(in_scatter_point, -ray_direction, step_size * f32(i));
        let transmittance = exp(-(sun_ray_optical_depth + view_ray_optical_depth));
        let local_density = density_at_point(in_scatter_point);

        in_scattered_light += transmittance * local_density * step_size;
        in_scatter_point += ray_direction * step_size;
    };
    return in_scattered_light;
}