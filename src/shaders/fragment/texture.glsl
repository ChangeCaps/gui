#version 450

uniform sampler2D tex;
uniform vec4 fill_color;
uniform vec2 texture_dimensions;

in vec2 v_tex_coords;

out vec4 color;

void main() {
    vec2 tex_coord = v_tex_coords;

    color = texelFetch(tex, ivec2(floor(tex_coord * texture_dimensions)), 0) * fill_color;
}
