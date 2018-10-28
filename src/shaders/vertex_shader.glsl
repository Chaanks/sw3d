#version 450

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv;


layout(location = 1) out vec2 v_tex_coords;


layout(set = 0, binding = 1) uniform Data {
    mat4 translation;
    mat4 rotation;
    mat4 scale;
} uniforms;


void main() {
    gl_Position =  uniforms.translation * uniforms.rotation * vec4(pos, 1.0);
    v_tex_coords = uv;
}
