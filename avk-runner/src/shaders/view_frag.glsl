#version 330
precision highp float;

out vec4 fragColor;
in highp vec2 texCoord;

uniform sampler2D sprite;

void main() {
    // fragColor = texture(sprite, texCoord);
    // fragColor = vec4(1.0, 1.0, 1.0, 1.0);
    vec2 adjustTexCoord = (texCoord + 1.0) * 0.5;
    fragColor = texture(sprite, adjustTexCoord);
    //fragColor = vec4(adjustTexCoord.x, adjustTexCoord.y, 0.0, 1.0);
}
