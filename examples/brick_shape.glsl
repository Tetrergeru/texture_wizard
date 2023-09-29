#version 330 core

#include "hash.glsl"
#include "random.glsl"
#include "perlin.glsl"

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

void main() {
    int scale = 4;
    vec2 pos = IN.Position.xy * scale;

    pos.x += scale;
    pos.y += scale;

    ivec2 ipos = ivec2(floor(pos));

    int cl = (ipos.x + ipos.y) % 2;

    if (cl == 0) {
        ipos.x -= 1;
    }

    // float pos_x = pos.x;
    if (ipos.x < 0) {
        ipos.x += scale + scale;
        pos.x  += scale + scale;
    }

    // float d_y = distance(pos.y, ipos.y + 0.5) * 2;
    // float d_x = distance(pos_x, ipos.x + 1);
    // float dist = max(d_y, d_x);

    vec2 uv = (pos - ipos) * vec2(0.5, 1);

    Color = vec4(uv, ipos.x / (scale * 2.0), ipos.y / (scale * 2.0));
}
