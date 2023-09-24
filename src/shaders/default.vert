#version 330 core

layout (location = 0) in vec3 Position;
layout (location = 1) in vec2 TextureCoords;

out VS_OUTPUT {
    vec2 TextureCoords;
    vec3 Position;
} OUT;

void main()
{
    gl_Position = vec4(Position, 1.0);
    OUT.TextureCoords = TextureCoords;
    OUT.Position = Position;
}