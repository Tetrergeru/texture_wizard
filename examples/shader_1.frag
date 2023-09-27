#version 330 core

in VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} IN;

out vec4 Color;

uniform sampler2D markup;

void main() {
    vec4 color = texture(markup, IN.TextureCoords);
    color.b = 1 - color.b;
    Color = 0.0001 * vec4(IN.TextureCoords, 0, 1) + color;
}
