#version 330 core

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

// uniform sampler2D noise;

void main() {
    Color = vec4(IN.TextureCoords, 0.0f, 1.0f); //texture(noise, IN.TextureCoords);
}