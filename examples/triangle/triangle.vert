#version 450

out gl_PerVertex {
    vec4 gl_Position;
};

// Ouput
layout(location = 0) out vec2 f_TexCoord;

void main() {
    vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1;
    vec2 texcoord = vec2(gl_VertexIndex / 2., gl_VertexIndex & 1);
    f_TexCoord = texcoord;
    gl_Position = vec4(position, 0., 1.0);
}