#version 450

layout(location = 0) in vec3 pos;
layout(location = 1) in vec2 uv;


layout(location = 1) out vec2 v_tex_coords;


layout(set = 0, binding = 1) uniform Data {
    mat4 model;
    mat4 projection;
    mat4 view;
} uniforms;


void main() {
    mat4 mvp =  uniforms.projection * uniforms.view * uniforms.model;
    gl_Position =  mvp * vec4(pos, 1.0);
    v_tex_coords = uv;
}
