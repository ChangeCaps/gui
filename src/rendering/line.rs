use super::super::*;
use math::*;

pub struct Line<'s> {
    p0: Vec2<f32>,
    p1: Vec2<f32>,
    width: f32,
    smooth: bool,
    color: [f32; 4],
    anchor: Anchor,
    scaling: bool,
    depth: f32,
    drawing_data: &'s mut DrawingData,
}

impl<'s> Line<'s> {
    pub fn new(data: &'s mut DrawingData) -> Self {
        Self {
            p0: Vec2::new(0.2, 0.2),
            p1: Vec2::new(-0.2, -0.2),
            width: 0.02,
            smooth: false,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            drawing_data: data,
        }
    }

    pub fn points(mut self, p0: Vec2<f32>, p1: Vec2<f32>) -> Self {
        self.p0 = p0;
        self.p1 = p1;
        self
    }

    pub fn draw(&mut self) {
        self.drawing_data.pixel_window_dimensions.map(|dims| {
            self.p0 /= dims.y / 2.0;
            self.p1 /= dims.y / 2.0;

            self.width /= dims.y;        
        }); 

        let a = (self.p1 - self.p0).normalize();
        
        let mut v0 = Vec2::new(a.y, -a.x) * self.width + self.p0;
        let mut v1 = Vec2::new(a.y, -a.x) * self.width + self.p1;
        let mut v2 = Vec2::new(-a.y, a.x) * self.width + self.p0;
        let mut v3 = Vec2::new(-a.y, a.x) * self.width + self.p1;

        if self.smooth {
            v0 += a * -self.width;
            v1 += a * self.width;
            v2 += a * -self.width;
            v3 += a * self.width;
        }

        let verts = &[
            v0,
            v1,
            v2,
            v1,
            v2,
            v3
        ];

        for vert in verts {
            let mut position = *vert;

            if self.scaling {
                position.x /= self.drawing_data.scaled_aspect_ratio;
            } else { 
                position.x /= self.drawing_data.aspect_ratio;
            }

            position += self.anchor.as_vec();

            self.drawing_data.verts.push(super::Vertex {
                position: position.as_array(),
                texture_coords: (*vert + 0.5).as_array(),
                color: self.color,
                depth: self.depth,
                shape: 2,
                shape_index: self.drawing_data.line_points.len() as i32,
                mask_length: 0,
                mask_index: 0,
            });
        }

        self.p0 += self.anchor.as_vec();
        self.p1 += self.anchor.as_vec();

        self.drawing_data.line_points.push([self.p0.x, self.p0.y, self.p1.x, self.p1.y]);
        self.drawing_data.line_widths.push(self.width);
    }
}

color!(Line);
anchor!(Line);
smooth!(Line);
width!(Line);
scaling!(Line);
depth!(Line);
