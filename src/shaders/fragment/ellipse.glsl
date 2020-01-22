#version 450

uniform vec2 pos;
uniform vec2 size;
uniform vec4 fill_color;
uniform float aspect_ratio;
uniform float scaled_aspect_ratio;
uniform bool scale_aspect_ratio;
uniform vec2 window_dimensions;
uniform vec2 anchor;
uniform vec2 pivot;

out vec4 color;

void main() {
    vec2 position = pos;
    vec2 pixel_position = gl_FragCoord.xy/window_dimensions * 2.0 - 1.0;

    position -= (pivot - 0.5) * size;

    if (scale_aspect_ratio) {
        position.x += anchor.x * scaled_aspect_ratio;
        position.y += anchor.y;

        pixel_position.x *= scaled_aspect_ratio;
    } else {
        position.x += anchor.x * aspect_ratio;
        position.y += anchor.y;

        pixel_position.x *= aspect_ratio;
    }

    if (length((pixel_position - position) / size) < 0.5) {
        color = fill_color;
    }
}
