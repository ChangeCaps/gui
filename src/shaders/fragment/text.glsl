#version 450

uniform vec4 fill_color;
uniform sampler2D tex;

in vec2 v_tex_coords;

out vec4 color;

void main() {
    vec4 c = vec4(fill_color.rgb, fill_color.a * texture(tex, v_tex_coords));

    if (c.a <= 0.1) {
        discard;
    } else {
        color = c;
    }
}