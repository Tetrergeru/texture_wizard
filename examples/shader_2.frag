#version 330 core

in VS_OUTPUT {
    vec2 TextureCoords;
} IN;

out vec4 Color;

void main() {
    Color = vec4(1.0f, 1.0f, 1.0f, 1.0f);
}
