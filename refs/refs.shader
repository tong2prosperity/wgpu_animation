#version 450

layout(location = 0) out vec4 outColor;

layout(location = 0) in vec2 vUV;  // 传入的纹理坐标

uniform vec2 p1, p2, p3, p4;       // 定义的四个点位置
uniform vec4 c1, c2, c3, c4;       // 定义的四种颜色

void main() {
    // 计算像素与每个点的距离
    float d1 = distance(vUV, p1);
    float d2 = distance(vUV, p2);
    float d3 = distance(vUV, p3);
    float d4 = distance(vUV, p4);

    // 距离越小，权重越大，这里用反比来计算权重
    float w1 = 1.0 / d1;
    float w2 = 1.0 / d2;
    float w3 = 1.0 / d3;
    float w4 = 1.0 / d4;

    // 归一化权重
    float sumWeights = w1 + w2 + w3 + w4;
    w1 /= sumWeights;
    w2 /= sumWeights;
    w3 /= sumWeights;
    w4 /= sumWeights;

    // 混合颜色
    outColor = w1 * c1 + w2 * c2 + w3 * c3 + w4 * c4;
}