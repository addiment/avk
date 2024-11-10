#version 330

layout (location = 0) in vec2 a_position;
out vec2 texCoord;

uniform vec2 pos;

void main() {
    /// for some reason, this has to be half the actual resolution.
    /// I think it has something to do with the fact that the viewport is [-1, 1]?
    /// I'm no good with math.
    const vec2 canvas_size = vec2(128.0, 96.0);
    const vec2 image_size = vec2(16.0, 16.0);
    gl_Position = vec4((a_position / canvas_size * image_size - 1.0) + (pos / canvas_size), 0.0, 1.0);
    texCoord = a_position;
}