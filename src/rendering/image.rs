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
    scale: f32,
    size: Option<Vec2<f32>>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>,
}

pub struct Image {
    image: String,
    position: Vec2<f32>,
    scale: f32,
    size: Option<Vec2<f32>>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
}

impl<'s> ImageBuilder<'s> {
    pub fn new(shapes: &'s mut Vec<(Box<dyn super::Shape>, f32)>, image: String) -> Self {
        Self {
            image,
            position: Vec2::new(0.0, 0.0),
            scale: 1.0,
            size: None,
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            shape_vec: shapes,
        }
    }

    pub fn size(mut self, size: Vec2<f32>) -> Self {
        self.size = Some(size);
        self
    }

    pub fn draw(self) {
        self.shape_vec.push((Box::new(Image {
            position: self.position,
            scale: self.scale,
            size: self.size,
            rotation: self.rotation,
            color: self.color,
            anchor: self.anchor,
            pivot: self.pivot,
            scaling: self.scaling,
            image: self.image,
        }), self.depth))
    }
}

impl super::Shape for Image {
    fn draw(&mut self, drawing_data: &mut DrawingData) {
        let image = drawing_data.images.get(&self.image).unwrap();

        // get image dimensions
        let tex_dims = image.dimensions();
        let tex_dims = (tex_dims.0 as f32, tex_dims.1 as f32);

        // calcualate the size of the image
        let mut size = if let Some(size) = self.size {
            size
        } else {
            Vec2::new(tex_dims.0/tex_dims.1, 1.0) * self.scale
        };

        // transform some variable if pixel mode
        drawing_data.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;

            let tex_dims = Vec2::new(tex_dims.0, tex_dims.1);

            size *= tex_dims.y / dims.y * 2.0;
        }); 

        let vertex_buffer = glium::VertexBuffer::new(drawing_data.display, RECT_VERTS)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(drawing_data.display, glium::index::PrimitiveType::TrianglesList, RECT_INDECIES)
            .expect("failed to create index buffer");      

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: size.as_array(),
            rotation: Mat2::<f32>::from_radians(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            pivot: (self.pivot.as_vec() / 2.0 + 0.5).as_array(),
            aspect_ratio: drawing_data.aspect_ratio,
            scaled_aspect_ratio: drawing_data.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: drawing_data.window_dimensions.as_array(),
            fill_color: self.color,
            texture_dimensions: tex_dims,
            tex: image,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(), 
            .. Default::default()
        };

        drawing_data.frame.as_surface().draw(
            &vertex_buffer, 
            &index_buffer, 
            drawing_data.texture,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(ImageBuilder);
rotation!(ImageBuilder);
color!(ImageBuilder);
anchor!(ImageBuilder);
pivot!(ImageBuilder);
scaling!(ImageBuilder);
depth!(ImageBuilder);
scale!(ImageBuilder);
