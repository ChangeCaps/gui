#version 450

// line
buffer line_point_buffer {
	vec4 line_points[];
};

buffer line_width_buffer {
	float line_widths[];
};

// image
uniform sampler2D image_atlas;
uniform vec2 image_atlas_dimensions;

// text
uniform sampler2D font_atlas;
uniform vec2 font_atlas_dimensions;

// rect mask
buffer rect_mask_position_buffer {
	vec2 rect_mask_positions[];
};

buffer rect_mask_size_buffer {
	vec2 rect_mask_sizes[];
};

buffer rect_mask_rotation_buffer {
	mat2 rect_mask_rotations[];
};

// masks
buffer mask_shape_buffer {
	ivec2 mask_shapes[];
};

uniform vec2 window_dimensions;
uniform float aspect_ratio;

in vec2 v_texture_coords;
in vec4 v_color;
flat in int v_shape;
flat in int v_shape_index;
flat in int v_mask_index;
flat in int v_mask_length;

out vec4 color;

vec2 min_distance_line(vec2 p0, vec2 p1, vec2 p) {
    const vec2 line = (p1 - p0);
    const float l2 = (line.y * line.y + line.x * line.x);

    if (l2 == 0.0) return (p1 - p);

    const float t = max(0, min(1, dot(p - p0, p1 - p0) / l2));
    const vec2 projection = p0 + t * (p1 - p0);
    return (p - projection);
}

vec4 ellipse() {
	if (v_texture_coords.x * v_texture_coords.x + v_texture_coords.y * v_texture_coords.y <= 0.25) {
		return v_color;
	}

	discard;
}

vec4 line(vec2 pos) {
	vec2 p0 = line_points[v_shape_index].xy;
	vec2 p1 = line_points[v_shape_index].zw;

    vec2 diff = min_distance_line(p0, p1, pos);

	diff.x *= aspect_ratio;

	if (diff.x * diff.x + diff.y * diff.y <= pow(line_widths[v_shape_index] / 2.0, 2)) {
		return v_color;
	}

	discard;
}

vec4 image() {
	return texelFetch(image_atlas, ivec2(floor(v_texture_coords * image_atlas_dimensions)), 0) * v_color;
}

vec4 font() {
	const vec4 c = vec4(v_color.rgb, v_color.a * texture(font_atlas, v_texture_coords));

	if (c.a >= 0.1) {
		return c;
	} else {
		discard;
	}
}

bool rect_mask(vec2 pos, int index) {
	vec2 position = rect_mask_positions[index];
	vec2 size = rect_mask_sizes[index];
	mat2 rotation = rect_mask_rotations[index];

	pos -= position;
	//pos *= -rotation;

	if (abs(pos.x) <= size.x / 2.0 && abs(pos.y) <= size.y / 2.0) {
		return true;
	}

	return false;
}

void main() {
	const vec2 pos = gl_FragCoord.xy/window_dimensions * 2.0 - 1.0;

	for (int i = v_mask_index; i < v_mask_index + v_mask_length; i++) {
		bool draw = true;	

		switch (mask_shapes[i].x) {
			case 0:
				draw = rect_mask(pos, mask_shapes[i].y);
				break;
		}

		if (!draw) {
			discard;
		}
	}

	switch (v_shape) {
		case 0:
			color = v_color;
			break;

		case 1:
			color = ellipse();
			break;
			
		case 2:
			color = line(pos);
			break;
		
		case 3:
			color = image();
			break;
		
		case 4:
			color = font();
			break;
	}
}
