use super::super::*;
use super::Vertex;
use math::*;
use glium;
use glium::Surface;

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct Line<'s, 'f> {
    p0: Vec2<f32>,
    p1: Vec2<f32>,
    width: f32,
    smooth: bool,
    color: [f32; 4],
    anchor: Anchor,
    scaling: bool,
    frame: &'s Frame<'f>,
}

impl<'s, 'f> Line<'s, 'f> {
    pub fn new(frame: &'s Frame<'f>) -> Self {
        Self {
            p0: Vec2::new(0.2, 0.2),
            p1: Vec2::new(-0.2, -0.2),
            width: 0.02,
            smooth: false,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            scaling: false,
            frame
        }
    }

    pub fn points(mut self, p0: Vec2<f32>, p1: Vec2<f32>) -> Self {
        self.p0 = p0;
        self.p1 = p1;
        self
    }

    pub fn draw(self) {
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
            Vertex{ position: v0.as_array()},
            Vertex{ position: v1.as_array()},
            Vertex{ position: v2.as_array()},
            Vertex{ position: v3.as_array()},
        ];

        let vertex_buffer = glium::VertexBuffer::new(self.frame.display, verts)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(self.frame.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");

        let uniforms = uniform!{
            p0: self.p0.as_array(),
            p1: self.p1.as_array(),
            width: self.width,
            anchor: self.anchor.as_vec().as_array(),
            aspect_ratio: self.frame.aspect_ratio,
            scaled_aspect_ratio: self.frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: self.frame.window_dimensions.as_array(),
            fill_color: self.color,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        self.frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            self.frame.no_transform_line,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

color!(Line);
anchor!(Line);
smooth!(Line);
width!(Line);
scaling!(Line);
