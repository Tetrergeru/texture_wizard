#version 330 core

#include "hash.glsl"
#include "random.glsl"
#include "perlin.glsl"

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

uniform ivec2 scale;

void main() {
    float scale_y = scale.y / 2.0;
    vec2 pos = IN.Position.xy * vec2(scale.x, scale_y);

    pos.x += scale.x;
    pos.y += scale_y;

    ivec2 ipos = ivec2(floor(pos));

    int cl = (ipos.x + ipos.y) % 2;

    if (cl == 0) {
        ipos.x -= 1;
    }

    if (ipos.x < 0) {
        ipos.x += scale.x * 2;
        pos.x  += scale.x * 2;
    }

    vec2 uv = (pos - ipos) * vec2(0.5, 1);

    Color = vec4(uv, ipos.x / (scale.x * 2.0), ipos.y / (scale_y * 2.0));
}
