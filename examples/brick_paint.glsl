#version 330 core

#include "random.glsl"
#include "hash.glsl"
#include "perlin.glsl"

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

uniform sampler2D brick_shape;

float simple_noise(vec2 pos) {
    ivec2 ipos = ivec2(floor(pos));

    float p00 = u_random(hash_2(ipos.x, ipos.y));
    float p01 = u_random(hash_2(ipos.x, ipos.y + 1));
    float p10 = u_random(hash_2(ipos.x + 1, ipos.y));
    float p11 = u_random(hash_2(ipos.x + 1, ipos.y + 1));

    float r0 = mix(p00, p10, pos.x - ipos.x);
    float r1 = mix(p01, p11, pos.x - ipos.x);
    float r = mix(r0, r1, pos.y - ipos.y);

    return r;
}

float fractal_noise(vec2 pos, int iters) {
    float res = 0;
    int scale = 1;

    for (int i = 0; i < iters; i++) {
        res += simple_noise(pos * scale) / scale;
        scale <<= 1;
    }
    
    float sum = 2.0 - 1.0 / scale;
    return res / sum;
}

float fractal_noise_2(vec2 pos, int iters) {
    float res = 0;

    for (int i = 0; i < iters; i++) {
        res += simple_noise(pos * (1 << i)) / (1 << i);
    }
    
    float sum = 2.0 - 1.0 / (1 << iters);
    return res / sum;
}

void main() {
    vec2 pos = IN.Position.xy;

    vec4 sample = texture(brick_shape, IN.TextureCoords);
    vec2 uv = sample.rg;
    vec2 brick = sample.ba;

    float thick = 0.1;
    float shape = min(1 - abs(uv.y * 2 - 1), 2 - abs(uv.x * 4 - 2));
    float noise = shape * fractal_noise(pos * 2, 2);

    int iters = 4;
    float scale = 4;
    noise = fractal_noise(pos * scale, iters);

    shape = uv.r * 0.0001 + noise;

    Color = vec4(shape, shape, shape, 1);
}