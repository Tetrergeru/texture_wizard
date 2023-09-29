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
vec2 scale = vec2(3, 6);

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

float simple_noise_wrap(vec2 pos, int wrap) {
    ivec2 ipos = ivec2(floor(pos));
    ivec2 inxt = ipos + ivec2(1, 1);
    if (ipos.x == 0) {
        ipos.x += wrap;
        pos.x += wrap;
    }
    if (ipos.y == 0) {
        ipos.y += wrap;
        pos.y += wrap;
    }

    float p00 = u_random(hash_2(ipos.x, ipos.y));
    float p01 = u_random(hash_2(ipos.x, inxt.y));
    float p10 = u_random(hash_2(inxt.x, ipos.y));
    float p11 = u_random(hash_2(inxt.x, inxt.y));

    float r0 = mix(p00, p10, pos.x - ipos.x);
    float r1 = mix(p01, p11, pos.x - ipos.x);
    float r = mix(r0, r1, pos.y - ipos.y);

    return r;
}

float fractal_noise(vec2 pos, int iters) {
    float res = 0;

    for (int i = 0; i < iters; i++) {
        res += simple_noise(pos * (1 << i)) / (1 << i);
    }
    
    float sum = 2.0 - 1.0 / (1 << iters);
    return res / sum;
}

void main() {
    vec2 pos = (IN.Position.xy + vec2(1, 1)) * 0.5;

    vec4 sample = texture(brick_shape, IN.TextureCoords);
    vec2 uv = sample.rg;
    ivec2 brick = ivec2(floor(sample.ba * 100));

    float thick = 0.1;
    float sc = 2.0 * scale.y / scale.x;
    float shape = min(sc * (1 - abs(uv.x * 2 - 1)), 1 - abs(uv.y * 2 - 1));
    shape = max(shape, mix(shape, simple_noise_wrap(pos * 128, 128), 0.1));

    vec3 color;
    if (shape > 0.15) {
        HashSeed = hash_2(brick.x, brick.y);
        color = vec3(1, 0, 0) * fractal_noise(uv * 4, 4);
    } else {
        color = vec3(1, 1, 1);
    }

    Color = vec4(color, 1);
}