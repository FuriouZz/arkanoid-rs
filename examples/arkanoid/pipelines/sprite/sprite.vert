#version 450

// Input - PerVertex
layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec2 v_TexCoord;

// Input - PerInstance
layout(location = 2) in uint i_Layer;
layout(location = 3) in vec3 i_Translation;
layout(location = 4) in vec3 i_Scaling;
layout(location = 5) in vec4 i_Rotation;
layout(location = 6) in vec4 i_Origin;

// Ouput
layout(location = 0) out vec2 f_TexCoord;
layout(location = 1) flat out uint f_Layer;

// Uniforms
layout (set = 0, binding = 1) uniform Set0 {
    mat4 u_Projection;
};
layout (set = 1, binding = 1) uniform Set1 {
    ivec2 u_AtlasSize;
};

void main() {
    vec2 ratio = i_Origin.zw / u_AtlasSize.xy;

    f_TexCoord = v_TexCoord * ratio;
    f_Layer = i_Layer;

    vec3 pos = v_Position.xyz;
    // Scaling from vec3
    pos *= vec3(i_Scaling.xy * i_Origin.zw, i_Scaling.z);
    pos += vec3(i_Origin.xy, 0.0);
    // Rotate from quaternion
    // https://www.geeks3d.com/20141201/how-to-rotate-a-vertex-by-a-quaternion-in-glsl/
    pos = pos + 2.0 * cross(i_Rotation.xyz, cross(i_Rotation.xyz, pos) + i_Rotation.w * pos);
    // Translate from vec3
    pos += i_Translation.xyz;

    gl_Position = u_Projection * vec4(pos, 1.0);
}