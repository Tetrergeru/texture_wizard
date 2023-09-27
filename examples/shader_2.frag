#version 330 core

#include "random.glsl"
#include "examples/perlin.glsl"

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

// uniform sampler2D noise;

void main() {
    // Color = texture(noise, IN.TextureCoords);
    // color.g = color.g / 2;
    // Color = color * 0.0001 + color_red();

    vec2 pos = IN.TextureCoords * 10;
    // ivec2 i_pos = ivec2(floor(pos));
    float cl = perlin(pos);

    Color = vec4(cl, cl, cl, 1);
}