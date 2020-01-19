use super::super::*;
use super::Vertex;
use math::*;
use glium;
use glium::Surface;

static RECT_VERTS: &[Vertex] = &[
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [1.0, 0.0] },
    Vertex { position: [0.0, 1.0] },
    Vertex { position: [0.0, 0.0] },
];

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct Ellipse<'s, 'f> {
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: [f32; 4],
    frame: &'s mut Frame<'f>,
    anchor: Anchor,
    pivot: Anchor,
    depth: f32,
    scaling: bool,
}

impl<'s, 'f> Ellipse<'s, 'f> {
    pub fn new(frame: &'s mut Frame<'f>) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.2, 0.2),
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            frame,
        }
    }

    pub fn draw(mut self) {
        self.frame.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            self.size /= dims.y / 2.0;
        }); 

        let vertex_buffer = glium::VertexBuffer::new(self.frame.display, RECT_VERTS)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(self.frame.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: self.size.as_array(),
            rotation: Mat2::<f32>::from_degrees(0.0).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            pivot: (self.pivot.as_vec() / 2.0 + 0.5).as_array(),
            aspect_ratio: self.frame.aspect_ratio,
            scaled_aspect_ratio: self.frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: self.frame.window_dimensions.as_array(),
            depth: self.depth,
            fill_color: self.color,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        self.frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            self.frame.simple_transform_ellipse,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(Ellipse);
size!(Ellipse);
color!(Ellipse);
anchor!(Ellipse);
pivot!(Ellipse);
scaling!(Ellipse);
depth!(Ellipse);