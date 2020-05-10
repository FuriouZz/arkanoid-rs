#version 450

// Input
layout(location = 0) in vec2 f_TexCoord;

// Output
layout(location = 0) out vec4 o_Color;

// Uniforms
layout(set = 0, binding = 0) uniform sampler s_Color;
layout(set = 1, binding = 0) uniform texture2DArray t_Color;

void main() {
    vec4 color = texture(sampler2DArray(t_Color, s_Color), vec3(f_TexCoord, 1.0));
    o_Color = vec4(color.rgb, 1.);
}