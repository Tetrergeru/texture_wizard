#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 2) in vec2 TextureCoords;

out VS_OUTPUT {
    vec2 TextureCoords;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.TextureCoords = TextureCoords;
}