#version 330

layout (location = 0) in vec2 a_position;
out vec2 texCoord;

uniform vec2 pos;
// TODO: flipping

void main() {
    gl_Position = vec4(a_position + pos, 0.0, 1.0);
    texCoord = a_position;
}