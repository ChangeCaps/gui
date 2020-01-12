#version 450

uniform vec2 p0;
uniform vec2 p1;
uniform vec4 fill_color;

out vec4 color;

void main() {
    color = fill_color;
}