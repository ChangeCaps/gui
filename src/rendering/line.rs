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
    color: [f32; 4],
    anchor: Anchor,
    scaling: bool,
    frame: &'s mut Frame<'f>,
}

impl<'s, 'f> Line<'s, 'f> {
    pub fn new(frame: &'s mut Frame<'f>) -> Self {
        Self {
            p0: Vec2::new(0.2, 0.2),
            p1: Vec2::new(-0.2, -0.2),
            width: 0.02,
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

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: [f32; 4]) -> Self {
        self.color = color;
        self
    }

    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = anchor;
        self
    }

    pub fn scaling(mut self) -> Self {
        self.scaling = true;
        self
    }

    pub fn draw(self) {
        let a = self.p1 - self.p0;
        
        let v0 = Vec2::new(a.y, -a.x) * self.width + self.p0;
        let v1 = Vec2::new(a.y, -a.x) * self.width + self.p1;
        let v2 = Vec2::new(-a.y, a.x) * self.width + self.p0;
        let v3 = Vec2::new(-a.y, a.x) * self.width + self.p1;

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
            rotation: Mat2::<f32>::from_degrees(0.0).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            scaling: self.scaling,
            window_dimensions: self.frame.window_dimensions,
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