#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;

struct GlitchEffect {
    progress: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    _padding: vec3<f32>
#endif
}

@group(0) @binding(2) var<uniform> settings: GlitchEffect;

fn hash(pos: vec2<f32>) -> f32 {
    return fract(sin(dot(pos, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}

fn random_color(uv: vec2<f32>) -> vec4<f32> {
    let r = hash(uv);
    let g = hash(uv + vec2<f32>(1.0, 0.0));
    let b = hash(uv + vec2<f32>(0.0, 1.0));

    return vec4<f32>(r, g, b, 1.0);
}

fn distance_squared(a: vec2<f32>, b: vec2<f32>) -> f32 {
    let diff = a - b;
    return dot(diff, diff);
}

fn to_pixalated(v: vec2<f32>) -> vec2<f32> {
    return floor(v / 16.0);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let origin = to_pixalated(vec2<f32>(640.0, 360.0));
    let bottom_left = to_pixalated(vec2<f32>(0.0, 0.0));
    let position = to_pixalated(in.position.xy);

    let distance = distance_squared(position, origin);
    let max_distance = distance_squared(bottom_left, origin);

    if distance < settings.progress * settings.progress * max_distance {
        return random_color(position) * 0.3;
    }

    return textureSample(screen_texture, texture_sampler, in.uv);
}

