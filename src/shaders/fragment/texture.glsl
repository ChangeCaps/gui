#version 450

uniform sampler2D tex;
uniform vec4 fill_color;

in vec2 v_tex_coords;

out vec4 color;

void main() {
    color = texture(tex, v_tex_coords) * fill_color;
}