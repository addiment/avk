#version 330
precision highp float;

out vec4 fragColor;
in highp vec2 texCoord;

uniform sampler2D sprite;

void main() {
    float red = texture(sprite, 1.0 - texCoord).r;
    fragColor = vec4(red * 64.0, 0.0, 0.0, 1.0);
}
