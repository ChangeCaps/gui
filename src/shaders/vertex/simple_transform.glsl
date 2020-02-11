#version 450

in vec2 position;
in vec2 texture_coords;
in vec4 color;
in int shape;
in int shape_index;
in int mask_index;
in int mask_length;

out vec2 v_texture_coords;
out vec4 v_color;
flat out int v_shape;
flat out int v_shape_index;
flat out int v_mask_index;
flat out int v_mask_length;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);

	v_texture_coords = texture_coords;
	v_color = color;
	v_shape = shape;
	v_shape_index = shape_index;
	v_mask_index = mask_index;
	v_mask_length = mask_length;
}
