#version 450

// Input
layout(location = 0) in vec2 f_TexCoord;

// Output
layout(location = 0) out vec4 o_Color;

// Uniforms
layout(set = 0, binding = 0) uniform sampler s_Color;
layout(set = 1, binding = 0) uniform texture2D t_Color;

void main() {
    vec4 color = texture(sampler2D(t_Color, s_Color), f_TexCoord);
    o_Color = vec4(color.rgb, 1.);
}