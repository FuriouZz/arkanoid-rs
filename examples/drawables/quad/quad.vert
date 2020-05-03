#version 450

// Input
layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec2 v_TexCoord;

// Ouput
layout(location = 0) out vec2 f_TexCoord;

void main() {
    f_TexCoord = v_TexCoord;
    gl_Position = vec4(v_Position, 1.0);
}