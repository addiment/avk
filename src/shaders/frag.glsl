#version 330
precision highp float;

out vec4 fragColor;
in highp vec2 texCoord;

uniform sampler2D sprite;

void main() {
    // fragColor = texture(sprite, texCoord);
    // fragColor = texture(sprite, texCoord);
    //fragColor = vec4(1.0, 1.0, 1.0, 1.0);
    fragColor = vec4(texCoord.x, texCoord.y, 0.0, 1.0);
}
