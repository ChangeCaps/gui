#version 450

// ellipse
buffer ellipse_position_buffer {
	vec2 ellipse_positions[];
};

buffer ellipse_size_buffer {
	vec2 ellipse_sizes[];
};

// line
buffer line_point_buffer {
	vec4 line_points[];
};

uniform vec2 window_dimensions;

in vec2 texture_coords;
in vec4 v_color;
flat in int v_shape;
flat in int v_index;

out vec4 color;

vec2 min_distance_line(vec2 p0, vec2 p1, vec2 p) {
    const vec2 line = (p1 - p0);
    const float l2 = (line.y * line.y + line.x * line.x);

    if (l2 == 0.0) return (p1 - p);

    const float t = max(0, min(1, dot(p - p0, p1 - p0) / l2));
    const vec2 projection = p0 + t * (p1 - p0);
    return (p - projection);
}

vec4 ellipse(vec2 pos) {
	const vec2 position = ellipse_positions[v_index];
	const vec2 size = ellipse_sizes[v_index];

	const vec2 diff = (pos - position) / size;

	if (diff.x * diff.x + diff.y * diff.y <= 0.25) {
		return v_color;
	}

	discard;
}

vec4 line(vec2 pos) {
    const vec2 diff = min_distance_line(line_points[v_index].zw, line_points[v_index].xy, pos);

	if (diff.x * diff.x + diff.y * diff.y <= 0.25) {
		return v_color;
	}

	discard;
}

void main() {
	const vec2 pos = gl_FragCoord.xy/window_dimensions * 2.0 - 1.0;

	switch (v_shape) {
		case 0:
			color = v_color;
			break;

		case 1:
			color = ellipse(pos);
			break;
			
		case 2:
			color = line(pos);
			break;
	}
}
