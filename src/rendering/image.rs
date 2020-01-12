use super::super::*;
use super::TextureVertex;
use math::*;
use glium;
use glium::Surface;

static RECT_VERTS: &[TextureVertex] = &[
    TextureVertex { position: [0.5, 0.5], texture_coords: [1.0, 1.0] },
    TextureVertex { position: [-0.5, 0.5], texture_coords: [0.0, 1.0] },
    TextureVertex { position: [0.5, -0.5], texture_coords: [1.0, 0.0] },
    TextureVertex { position: [-0.5, -0.5], texture_coords: [0.0, 0.0] },
];

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct Image<'s, 'f> {
    image: &'s super::super::Image,
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    frame: &'s mut Frame<'f>,
    anchor: Anchor,
    scaling: bool,
}

impl<'s, 'f> Image<'s, 'f> {
    pub fn new(frame: &'s mut Frame<'f>, image: &'s super::super::Image) -> Self {
        let image_dimensions = image.dimensions();
        let dimension_ratio = image_dimensions.x/image_dimensions.y;

        Self {
            image,
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.2 * dimension_ratio, 0.2),
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            scaling: false,
            frame
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        let image_dimensions = self.image.dimensions();
        let dimension_ratio = image_dimensions.x/image_dimensions.y;

        self.size = Vec2::new(scale * dimension_ratio, scale);

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
            aspect_ratio: self.frame.aspect_ratio,
            scaled_aspect_ratio: self.frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: self.frame.window_dimensions.as_array(),
            fill_color: self.color,
            tex: &self.image.texture,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        self.frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            self.frame.texture,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(Image);
size!(Image);
rotation!(Image);
color!(Image);
anchor!(Image);
scaling!(Image);