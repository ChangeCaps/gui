use super::super::*;
use super::Vertex;
use math::*;
use glium;
use glium::Surface;

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct LineBuilder<'s> {
    p0: Vec2<f32>,
    p1: Vec2<f32>,
    width: f32,
    smooth: bool,
    color: [f32; 4],
    anchor: Anchor,
    scaling: bool,
    depth: f32,
    shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>,
}

pub struct Line {
    p0: Vec2<f32>,
    p1: Vec2<f32>,
    width: f32,
    smooth: bool,
    color: [f32; 4],
    anchor: Anchor,
    scaling: bool,
    depth: f32,
}

impl<'s> LineBuilder<'s> {
    pub fn new(shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>) -> Self {
        Self {
            p0: Vec2::new(0.2, 0.2),
            p1: Vec2::new(-0.2, -0.2),
            width: 0.02,
            smooth: false,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            shape_vec
        }
    }

    pub fn points(mut self, p0: Vec2<f32>, p1: Vec2<f32>) -> Self {
        self.p0 = p0;
        self.p1 = p1;
        self
    }

    pub fn draw(self) {
        self.shape_vec.push((Box::new(Line {
            p0: self.p0,
            p1: self.p1,
            color: self.color,
            anchor: self.anchor,
            scaling: self.scaling,
            depth: self.depth,
            smooth: self.smooth,
            width: self.width,
        }), self.depth))
    }
}

impl super::Shape for Line {
    fn draw(&mut self, frame: &mut Frame) {
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

        let vertex_buffer = glium::VertexBuffer::new(frame.display, verts)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(frame.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");

        let uniforms = uniform!{
            p0: self.p0.as_array(),
            p1: self.p1.as_array(),
            width: self.width,
            anchor: self.anchor.as_vec().as_array(),
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
            frame.no_transform_line,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

color!(LineBuilder);
anchor!(LineBuilder);
smooth!(LineBuilder);
width!(LineBuilder);
scaling!(LineBuilder);
depth!(LineBuilder);