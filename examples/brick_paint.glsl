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
uniform ivec2 scale;
uniform vec3 brick_color;

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

vec2 vector(ivec2 pos) {
    float x = s_random(hash_3(pos.x, pos.y, 0));
    float y = s_random(hash_3(pos.x, pos.y, 1)) * sqrt(1.0 - x * x);
    return vec2(x, y) * 0.5 + vec2(0.5);
}

float magic_number(ivec2 pos) {
    return u_random(hash_3(pos.x, pos.y, 0));
}

float voronoi(vec2 pos) {
    ivec2 ipos = ivec2(floor(pos));

    float minDist = 1.0;

    vec2 minPos = ipos;

    for (int i = -1; i <= 1; i++)
    for (int j = -1; j <= 1; j++) {
        ivec2 iposOfs = ipos + ivec2(i, j);
        if (magic_number(iposOfs) < 0.95) continue;
        vec2 offset = vector(iposOfs);

        float dist = distance(pos, vec2(iposOfs) + offset);

        minPos = minDist < dist ? minPos : iposOfs;
        minDist = min(minDist, dist);
    }
    
    return minDist;

}

void main() {
    vec2 pos = (IN.Position.xy + vec2(1, 1)) * 0.5;

    vec4 sample = texture(brick_shape, IN.TextureCoords);
    vec2 uv = sample.rg;
    ivec2 brick = ivec2(floor(sample.ba * 100));

    float thick = 0.1;
    float sc = scale.y / scale.x;
    float shape = min(sc * (1 - abs(uv.x * 2 - 1)), 1 - abs(uv.y * 2 - 1));
    shape = mix(shape, simple_noise_wrap(pos * 128, 128), 0.1);
    
    vec3 color;
    if (shape > 0.12) {
        HashSeed = hash_2(brick.x, brick.y);

        float a = s_random(hash_1(1));
        mat2 rot = mat2(cos(a), -sin(a), sin(a), cos(a));
        float v = voronoi(rot * uv * vec2(2, 10));
        color = brick_color * fractal_noise(uv * 7, 10) * v;
    } else {
        HashSeed = 1;
        float noise = fractal_noise(pos * 256, 5) * 0.7 + 0.3;
        color = vec3(1, 1, 1) * noise;
    }

    Color = vec4(color, 1);
}