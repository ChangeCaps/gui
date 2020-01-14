#version 450

uniform mat2 rotation;
uniform vec2 pos;
uniform vec2 size;
uniform float aspect_ratio;
uniform float scaled_aspect_ratio;
uniform bool scale_aspect_ratio;
uniform vec2 window_dimensions;
uniform vec2 anchor;

in vec2 position;
in vec2 texture_coords;

out vec2 v_tex_coords;

void main() {
    vec2 vertex_position = position * size * rotation + pos;

    if (scale_aspect_ratio) {
        vertex_position.x /= scaled_aspect_ratio;
    } else {
        vertex_position.x /= aspect_ratio;
    }

    vertex_position += anchor;

    gl_Position = vec4(position, 0.0, 1.0);
    v_tex_coords = texture_coords;
}