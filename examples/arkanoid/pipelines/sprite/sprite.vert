#version 450

// Input
layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec2 v_TexCoord;

// Ouput
layout(location = 0) out vec2 f_TexCoord;

// Uniforms
layout (set = 0, binding = 1) uniform Data {
    mat4 u_Transform;
};

void main() {
    f_TexCoord = v_TexCoord;
    gl_Position = u_Transform * vec4(v_Position, 1.0);
}