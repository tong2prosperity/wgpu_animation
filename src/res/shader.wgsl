struct VertexInput {
@location(0) position: vec3<f32>,
@location(1) color: vec3<f32>,
}

struct Uniforms {
    center: vec2<f32>,
    radius: f32,
    feather: f32,
    color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.color = model.color;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.uv = model.position.xy;

    return out;
}

fn custom_distance(p1: vec2<f32>, p2: vec2<f32>) -> f32 {
    let diff = p1 - p2;
    return sqrt(diff.x * diff.x + diff.y * diff.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = distance(in.uv, uniforms.center);
    let alpha = 1.0 - smoothstep(0.0, 1.0, dist);
    //return vec4<f32>(uniforms.color.rgb * alpha, uniforms.color.a * alpha);
    return vec4<f32>(in.color.rgb * alpha, uniforms.color.a * alpha);
}
 