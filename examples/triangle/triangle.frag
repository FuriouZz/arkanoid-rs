#version 450

// Input
layout(location = 0) in vec3 f_Color;

// Output
layout(location = 0) out vec4 o_Color;

void main() {
    o_Color = vec4(f_Color, 1.);
}