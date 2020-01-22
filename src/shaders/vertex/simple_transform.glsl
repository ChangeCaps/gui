#version 450

uniform vec2 pos;
uniform vec2 size;
uniform mat2 rotation;
uniform float aspect_ratio;
uniform float scaled_aspect_ratio;
uniform bool scale_aspect_ratio;
uniform vec2 window_dimensions;
uniform vec2 anchor;
uniform vec2 pivot;

in vec2 position;

void main() {
    vec2 vertex_position = (position - pivot) * size * rotation + pos;

    if (scale_aspect_ratio) {
        vertex_position.x /= scaled_aspect_ratio;
    } else {
        vertex_position.x /= aspect_ratio;
    }

    vertex_position += anchor;

    gl_Position = vec4(vertex_position, 0.0, 1.0);
}