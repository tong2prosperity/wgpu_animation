struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) uv: vec2<f32>,
};



struct ColorPoint {
    position: vec3<f32>,
    color: vec4<f32>,
};

struct CPUniforms {
    color_points: array<ColorPoint, 4>,
};

@group(0) @binding(0) var<uniform> cp: CPUniforms;


struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    return VertexOutput(vec4<f32>(model.position, 1.0), model.color, model.position.xy);
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var final_color: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    let d1 = distance(in.uv, cp.color_points[0].position.xy);
    let d2 = distance(in.uv, cp.color_points[1].position.xy);
    let d3 = distance(in.uv, cp.color_points[2].position.xy);
    let d4 = distance(in.uv, cp.color_points[3].position.xy);

    let sumWeights = d1 + d2 + d3 + d4;

    var w1 = d1 / sumWeights;
    var w2 = d2 / sumWeights;
    var w3 = d3 / sumWeights;
    var w4 = d4 / sumWeights;


    // 
    // w1 /= sumWeights;
    // w2 /= sumWeights;
    // w3 /= sumWeights;
    // w4 /= sumWeights;

    final_color = cp.color_points[0].color * w1 + cp.color_points[1].color * w2 + cp.color_points[2].color * w3 + cp.color_points[3].color * w4;


    // final_color = cp.color_points[0].color * w1 + cp.color_points[1].color * w2 + cp.color_points[2].color * w3 + cp.color_points[3].color * w4;
    // total_weight = w1 + w2 + w3 + w4;

    // for (var i: i32 = 0; i < 4; i++) {
    //     let dist = distance(in.uv, cp.color_points[i].position.xy);
    //     let weight = 1.0 / (dist * dist + 0.00001); // 避免除以零
    //     final_color += cp.color_points[i].color * weight;
    //     total_weight += weight;
    // }

    return final_color;
}