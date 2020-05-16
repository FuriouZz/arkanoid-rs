#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

// Ouput
layout(location = 0) out vec3 f_Color;

void main() {
    vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1;
    vec3 color = vec3((min(gl_VertexIndex+1, 2) & 1), (gl_VertexIndex & 1), max(gl_VertexIndex-1, 0));

    f_Color = color;
    gl_Position = vec4(position, 0., 1.0);
}