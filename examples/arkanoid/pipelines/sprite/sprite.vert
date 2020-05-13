#version 450

// Input - PerVertex
layout(location = 0) in vec3 v_Position;
layout(location = 1) in vec2 v_TexCoord;

// Input - PerInstance
layout(location = 2) in vec3 i_LayerAndOrigin;
layout(location = 3) in vec4 i_LayerRect;
layout(location = 4) in vec3 i_Translation;
layout(location = 5) in vec3 i_Scaling;
layout(location = 6) in vec4 i_Rotation;

// Ouput
layout(location = 0) out vec2 f_TexCoord;
layout(location = 1) out float f_Layer;

// Uniforms
layout (set = 0, binding = 1) uniform Set0 {
    mat4 u_Projection;
};
layout (set = 1, binding = 1) uniform Set1 {
    vec2 u_AtlasSize;
};

void main() {
    vec2 offset = i_LayerRect.xy / u_AtlasSize.xy;
    vec2 ratio = i_LayerRect.zw / u_AtlasSize.xy;

    f_TexCoord = (v_TexCoord * ratio) + offset;
    f_Layer = i_LayerAndOrigin.x;

    vec3 pos = v_Position.xyz - vec3(i_LayerAndOrigin.yz, 0.0);
    // Scaling from vec3
    pos *= vec3(i_Scaling.xy * i_LayerRect.zw, i_Scaling.z);
    // Rotate from quaternion
    // https://www.geeks3d.com/20141201/how-to-rotate-a-vertex-by-a-quaternion-in-glsl/
    pos = pos + 2.0 * cross(i_Rotation.xyz, cross(i_Rotation.xyz, pos) + i_Rotation.w * pos);
    // Translate from vec3
    pos += i_Translation.xyz;

    gl_Position = u_Projection * vec4(pos, 1.0);
}