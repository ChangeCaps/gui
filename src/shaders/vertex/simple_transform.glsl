#version 450

in vec2 position;
in vec2 texture_coords;
in vec4 color;
in float depth;
in int shape;
in int index;

out vec2 v_texture_coords;
out vec4 v_color;
flat out int v_shape;
flat out int v_index;

void main() {
    gl_Position = vec4(position, depth, 1.0);

	v_texture_coords = texture_coords;
	v_color = color;
	v_shape = shape;
	v_index = index;
}
