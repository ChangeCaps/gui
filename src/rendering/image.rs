use super::super::*;
use super::TextureVertex;
use math::*;
use glium;
use glium::Surface;

static RECT_VERTS: &[TextureVertex] = &[
    TextureVertex { position: [1.0, 1.0], texture_coords: [1.0, 1.0] },
    TextureVertex { position: [0.0, 1.0], texture_coords: [0.0, 1.0] },
    TextureVertex { position: [1.0, 0.0], texture_coords: [1.0, 0.0] },
    TextureVertex { position: [0.0, 0.0], texture_coords: [0.0, 0.0] },
];

static RECT_INDECIES: &[u32] = &[0, 1, 2, 1, 2, 3];

pub struct ImageBuilder<'s> {
    image: String,
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    ratio: f32,
    shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>,
}

pub struct Image {
    image: String,
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
}

impl<'s> ImageBuilder<'s> {
    pub fn new(frame: &'s mut Frame, image: String) -> Self {
        let img = frame.images.get(&image).unwrap();

        let image_dimensions = img.dimensions();
        let dimension_ratio = image_dimensions.0 as f32/image_dimensions.1 as f32;

        Self {
            image,
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(1.0 * dimension_ratio, 1.0),
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            ratio: dimension_ratio,
            depth: 0.0,
            shape_vec: &mut frame.shapes,
        }
    }

    pub fn scale(mut self, scale: f32) -> Self {
        self.size = Vec2::new(scale * self.ratio, scale);
        self
    }

    pub fn draw(self) {
        self.shape_vec.push((Box::new(Image {
            position: self.position,
            size: self.size,
            rotation: self.rotation,
            color: self.color,
            anchor: self.anchor,
            pivot: self.pivot,
            scaling: self.scaling,
            depth: self.depth,
            image: self.image,
        }), self.depth))
    }
}

impl super::Shape for Image {
    fn draw(&mut self, frame: &mut Frame) {
        let image = frame.images.get(&self.image).unwrap();

        let tex_dims = image.dimensions();
        let tex_dims = (tex_dims.0 as f32, tex_dims.1 as f32);

        frame.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;

            let tex_dims = Vec2::new(tex_dims.0, tex_dims.1);

            self.size *= tex_dims.y / dims.y * 2.0;
        }); 

        let vertex_buffer = glium::VertexBuffer::new(frame.display, RECT_VERTS)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(frame.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");

        

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: self.size.as_array(),
            rotation: Mat2::<f32>::from_radians(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            pivot: (self.pivot.as_vec() / 2.0 + 0.5).as_array(),
            aspect_ratio: frame.aspect_ratio,
            scaled_aspect_ratio: frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: frame.window_dimensions.as_array(),
            fill_color: self.color,
            texture_dimensions: tex_dims,
            tex: image,
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
            frame.texture,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(ImageBuilder);
size!(ImageBuilder);
rotation!(ImageBuilder);
color!(ImageBuilder);
anchor!(ImageBuilder);
pivot!(ImageBuilder);
scaling!(ImageBuilder);
depth!(ImageBuilder);
