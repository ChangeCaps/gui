#version 450

uniform vec2 p0;
uniform vec2 p1;
uniform vec4 fill_color;
uniform float aspect_ratio;
uniform float scaled_aspect_ratio;
uniform bool scale_aspect_ratio;
uniform vec2 window_dimensions;
uniform vec2 anchor;
uniform float width;

out vec4 color;

float min_distance(vec2 p0, vec2 p1, vec2 p) {
    const vec2 line = (p1 - p0);
    const float l2 = (line.y * line.y + line.x * line.x);

    if (l2 == 0.0) return length(p1 - p);

    const float t = max(0, min(1, dot(p - p0, p1 - p0) / l2));
    const vec2 projection = p0 + t * (p1 - p0);
    return length(p - projection);
}

void main() {
    vec2 p0 = p0;
    vec2 p1 = p1;
    vec2 position = gl_FragCoord.xy/window_dimensions * 2.0 - 1.0;

    if (scale_aspect_ratio) {
        p0.x += anchor.x * scaled_aspect_ratio;
        p0.y += anchor.y;
        p1.x += anchor.x * scaled_aspect_ratio;
        p1.y += anchor.y;

        position.x *= scaled_aspect_ratio;
    } else {
        p0.x += anchor.x * aspect_ratio;
        p0.y += anchor.y;
        p1.x += anchor.x * aspect_ratio;
        p1.y += anchor.y;

        position.x *= aspect_ratio;
    }
    
    if (min_distance(p0, p1, position) < width) {
        color = fill_color;
    }
}
