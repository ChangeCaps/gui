#version 450

// line
buffer line_point_buffer {
	vec4 line_points[];
};

// image
uniform sampler2D image_atlas;
uniform vec2 image_atlas_dimensions;

//
uniform sampler2D font_atlas;
uniform vec2 font_atlas_dimensions;

uniform vec2 window_dimensions;

in vec2 v_texture_coords;
in vec4 v_color;
flat in int v_shape;
flat in int v_shape_index;

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
    const vec2 diff = min_distance_line(line_points[v_shape_index].zw, line_points[v_shape_index].xy, pos);

	if (diff.x * diff.x + diff.y * diff.y <= 0.25) {
		return v_color;
	}

	discard;
}

vec4 image() {
	return texelFetch(image_atlas, ivec2(floor(v_texture_coords * image_atlas_dimensions)), 0) * v_color;
}

vec4 font() {
	vec4 c = vec4(v_color.rgb, v_color.a * texture(font_atlas, v_texture_coords));

	if (c.a >= 0.1) {
		return c;
	} else {
		discard;
	}
}

void main() {
	const vec2 pos = gl_FragCoord.xy/window_dimensions * 2.0 - 1.0;

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
