struct VertexInput {
@location(0) position: vec2<f32>,
//@location(1) color: vec3<f32>,
}

struct Uniforms {
    center: vec2<f32>,
    radius: f32,
    feather: f32,
    color: vec4<f32>,
};

struct ActionMatrix {
    theta: f32,
    action_mat: mat3x3<f32>,
}

struct MVPMatrix{
    mvp: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(0) var<uniform> action_matrix: ActionMatrix;
@group(2) @binding(0) var<uniform> mvp_matrix: MVPMatrix;


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
};


fn rotate2D(angle: f32) -> mat3x3<f32> {
    let rotate = mat3x3<f32>(
        cos(angle), -sin(angle), 0.0,
        sin(angle), cos(angle), 0.0,
        0.0, 0.0, 1.0
    );
    return rotate;
 }

fn rotate_around_point(pos: vec2<f32>, center: vec2<f32>, angle: f32) -> vec2<f32> {
    // 将位置从中心点平移到原点
    var translated_pos = pos - center;

    // 构建旋转矩阵
    let cos_angle = cos(angle);
    let sin_angle = sin(angle);
    let rotation_matrix = mat2x2<f32>(
        cos_angle, -sin_angle,
        sin_angle, cos_angle
    );

    // 进行旋转并缩放到半径 r
    translated_pos = rotation_matrix * translated_pos;

    // 将位置从原点平移回中心点
    return translated_pos + center;
}


@vertex
fn vs_main(
    model: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.color = vec3<f32>(0.0, 0.5, 0.0);
    //let pos = rotate2D(action_matrix.theta) * model.position;
    let pos = rotate_around_point(model.position.xy, vec2<f32>(0.4, 0.2), action_matrix.theta);
    out.clip_position = mvp_matrix.mvp * vec4<f32>(pos.xy,1.0, 1.0);
    //out.uv = model.position.xy;
    out.uv = (vec3(model.position.xy, 1.0) * action_matrix.action_mat).xy;

    return out;
}

fn custom_distance(p1: vec2<f32>, p2: vec2<f32>) -> f32 {
    let diff = p1 - p2;
    return sqrt(diff.x * diff.x + diff.y * diff.y);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let dist = distance(in.uv, uniforms.center);
    let alpha = 1- smoothstep(0.0, 1.0, dist);
    return vec4<f32>(in.color.rgb , 1.0);
    //return vec4<f32>(in.color.rgb * alpha, uniforms.color.a * alpha);
}
