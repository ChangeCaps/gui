#version 450

uniform vec2 window_dimensions;
uniform vec2 anchor;
uniform bool scaling;

in vec2 position;

void main() {
    vec2 vertex_position = position;

    if (!scaling) {
        vertex_position *= (window_dimensions/max(window_dimensions.x, window_dimensions.y)).yx;
        vertex_position *= 1080.0 / min(window_dimensions.x, window_dimensions.y);
    }

    vertex_position += anchor;

    gl_Position = vec4(vertex_position, 0.0, 1.0);
}