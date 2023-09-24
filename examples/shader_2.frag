#version 330 core

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

uniform sampler2D noise;

void main() {
    Color = texture(noise, IN.TextureCoords);
}