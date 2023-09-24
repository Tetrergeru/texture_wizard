#version 330 core

in VS_OUTPUT {
    vec2 TextureCoords;
} IN;

out vec4 Color;

uniform sampler2D markup;

void main() {
    Color = texture(markup, IN.TextureCoords);
}
