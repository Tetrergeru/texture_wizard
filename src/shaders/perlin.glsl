#include "random.glsl"

float smoothstep(float x) {
    float edge0 = 0.0f;
    float edge1 = 1.0f;
    x = clamp((x - edge0) / (edge1 - edge0), 0, 1);

    return x * x * x * (3.0f * x * (2.0f * x - 5.0f) + 10.0f);
}

vec2 perlin_vec(ivec2 p) {
    float x = s_random(hash_3(p.x, p.y, 1));
    float y = s_random(hash_3(p.x, p.y, 2)) * sqrt(1 - x * x);
    return vec2(x, y);
}

float perlin_color(ivec2 pos) {
    return u_random(hash_3(pos.x, pos.y, 4));
}

float perlin(vec2 pos) {
    ivec2 ipos = ivec2(round(pos.x + 0.5), round(pos.y + 0.5));

    ivec2 pos00 = ivec2(ipos.x - 1, ipos.y - 1);
    ivec2 pos01 = ivec2(ipos.x - 1, ipos.y    );
    ivec2 pos11 = ivec2(ipos.x    , ipos.y    );
    ivec2 pos10 = ivec2(ipos.x    , ipos.y - 1);

    float dot00 = dot(perlin_vec(pos00), pos - pos00);
    float dot01 = dot(perlin_vec(pos01), pos - pos01);
    float dot11 = dot(perlin_vec(pos11), pos - pos11);
    float dot10 = dot(perlin_vec(pos10), pos - pos10);

    vec2 larg = pos - pos00;

    float top = mix(dot01, dot11, smoothstep(larg.x));
    float bot = mix(dot00, dot10, smoothstep(larg.x));
    float mid = mix(bot, top, smoothstep(larg.y));

    return (mid + 1) / 2;
}
