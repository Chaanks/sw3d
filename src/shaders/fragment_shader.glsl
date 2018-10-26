#version 450

layout(location = 1) in vec2 v_tex_coords;

layout(location = 0) out vec4 f_color;


layout(set = 0, binding = 0) uniform sampler2D tex;


void main() {
    f_color = texture(tex, v_tex_coords);
}
