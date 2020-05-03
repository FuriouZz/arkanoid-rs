#version 450

// Input
layout(location = 0) in vec2 f_TexCoord;

// Output
layout(location = 0) out vec4 o_Color;

void main() {
    o_Color = vec4(f_TexCoord, 0., 1.);
}