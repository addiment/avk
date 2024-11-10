#version 330
precision highp float;

out vec4 fragColor;
in highp vec2 texCoord;

uniform sampler2D sprite;
uniform vec2 flip;
uniform vec4 palette_0;
uniform vec4 palette_1;
uniform vec4 palette_2;
uniform vec4 palette_3;
uniform vec4 palette_4;
uniform vec4 palette_5;
uniform vec4 palette_6;
uniform vec4 palette_7;

void main() {
    //    uint red = uint(texture(sprite, 1.0 - texCoord).r * 15.0);
    uint red = uint(round(texture(sprite, vec2(texCoord.x * flip.x, -texCoord.y * flip.y)).r * 255.0));
    vec4[8] palettes = vec4[8](
        palette_0,
        palette_1,
        palette_2,
        palette_3,
        palette_4,
        palette_5,
        palette_6,
        palette_7
    );
    fragColor = palettes[red];
}
