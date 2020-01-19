use super::super::*;
use super::Vertex;
use math::*;
use glium;
use glium::Surface;

static RECT_VERTS: &[Vertex] = &[
    Vertex { position: [1.0, 1.0] },
    Vertex { position: [0.0, 1.0] },
    Vertex { position: [1.0, 0.0] },
    Vertex { position: [0.0, 0.0] },
];

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct RectBuilder<'s> {
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>,
}

pub struct Rect {
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
}

impl<'s> RectBuilder<'s> {
    pub fn new(shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.2, 0.2),
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            shape_vec
        }
    }

    pub fn draw(self) {
        self.shape_vec.push((Box::new(Rect {
            position: self.position,
            size: self.size,
            rotation: self.rotation,
            color: self.color,
            anchor: self.anchor,
            pivot: self.pivot,
            scaling: self.scaling,
            depth: self.depth,
        }), self.depth))
    }
}

impl super::Shape for Rect {
    fn draw<'f>(&mut self, frame: &mut Frame<'f>) {
        frame.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            self.size /= dims.y / 2.0;
        }); 

        let vertex_buffer = glium::VertexBuffer::new(frame.display, RECT_VERTS)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(frame.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: self.size.as_array(),
            rotation: Mat2::<f32>::from_degrees(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            pivot: (self.pivot.as_vec() / 2.0 + 0.5).as_array(),
            aspect_ratio: frame.aspect_ratio,
            scaled_aspect_ratio: frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: frame.window_dimensions.as_array(),
            fill_color: self.color,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            depth: glium::Depth {
                test: glium::DepthTest::IfMoreOrEqual,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            frame.simple_transform_fill,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}


position!(RectBuilder);
size!(RectBuilder);
rotation!(RectBuilder);
color!(RectBuilder);
anchor!(RectBuilder);
pivot!(RectBuilder);
scaling!(RectBuilder);
depth!(RectBuilder);
