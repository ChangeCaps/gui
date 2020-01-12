use super::super::*;
use super::Vertex;
use math::*;
use glium;
use glium::Surface;

static RECT_VERTS: &[Vertex] = &[
    Vertex { position: [0.5, 0.5] },
    Vertex { position: [0.5, -0.5] },
    Vertex { position: [-0.5, 0.5] },
    Vertex { position: [-0.5, -0.5] },
];

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct Rect<'s, 'f> {
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    frame: &'s mut Frame<'f>,
    anchor: Anchor,
    scaling: bool,
}

impl<'s, 'f> Rect<'s, 'f> {
    pub fn new(frame: &'s mut Frame<'f>) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.2, 0.2),
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            scaling: false,
            frame
        }
    }

    pub fn position(mut self, position: Vec2<f32>) -> Self {
        self.position = position;
        self
    }

    pub fn size(mut self, size: Vec2<f32>) -> Self {
        self.size = size;
        self
    }

    pub fn rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
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
        let vertex_buffer = glium::VertexBuffer::new(self.frame.display, RECT_VERTS)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(self.frame.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: self.size.as_array(),
            rotation: Mat2::<f32>::from_degrees(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            scaling: self.scaling,
            window_dimensions: self.frame.window_dimensions,
            scale_aspect_ratio: self.frame.scale_aspect_ratio,
            fill_color: self.color,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        self.frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            self.frame.fill_polygon,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}