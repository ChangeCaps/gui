use crate::{
    rendering::*,
    math::*,
};
use glium::{
    texture::texture2d::Texture2d,
    Surface,
};
use super::super::*;

type Shapes = Vec<(Box<dyn Shape>, f32)>;

pub trait Canvas {
    fn get_shapes(&mut self) -> &mut Shapes;

    fn rect(&mut self) -> RectBuilder {
        RectBuilder::new(self.get_shapes())
    }

    fn line(&mut self) -> LineBuilder {
        LineBuilder::new(self.get_shapes())
    }

    fn ellipse(&mut self) -> EllipseBuilder {
        EllipseBuilder::new(self.get_shapes())
    }

    fn image<P>(&mut self, image: P) -> ImageBuilder
        where P: Into<String>
    {
        ImageBuilder::new(self.get_shapes(), image.into())
    }

    fn text<P>(&mut self, font: P) -> TextBuilder
        where P: Into<String>
    {
        TextBuilder::new(self.get_shapes(), font.into())
    }

    fn new_frame(&mut self) -> Frame {
        Frame {
            shapes: Vec::new(),
        }
    }
}

pub struct Frame { 
    pub(crate) shapes: Vec<(Box<dyn Shape>, f32)>,
}

impl Frame {
    pub fn draw_frame(&mut self, frame: Frame) -> FrameDrawerBuilder {
        FrameDrawerBuilder::new(self, frame.shapes)
    }
}

impl Canvas for Frame {
    fn get_shapes(&mut self) -> &mut Shapes {
        &mut self.shapes
    }
}

pub struct FrameDrawerBuilder<'f> {
    position: Vec2<f32>,
    scale: f32,
    size: Option<Vec2<f32>>,
    dimensions: Vec2<u32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    parent_frame: &'f mut Frame,
    shapes: Shapes,
    pixel_mode: bool,
}

pub struct FrameDrawer {
    position: Vec2<f32>,
    scale: f32,
    size: Option<Vec2<f32>>,
    dimensions: Vec2<u32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    shapes: Vec<(Box<dyn Shape>, f32)>,
    pixel_mode: bool,
}

impl<'s> FrameDrawerBuilder<'s> {
    pub fn new(frame: &'s mut Frame, shapes: Vec<(Box<dyn Shape>, f32)>) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            scale: 1.0,
            size: None,
            dimensions: Vec2::new(100, 100),
            rotation: 0.0,
            color: [1.0; 4],
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            parent_frame: frame,
            shapes,
            pixel_mode: false,
        }
    }

    pub fn size(mut self, size: Vec2<f32>) -> Self {
        self.size = Some(size);
        self
    }

    pub fn dimensions(mut self, dimensions: Vec2<u32>) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn draw(self) {
        self.parent_frame.shapes.push((Box::new(FrameDrawer {
            position: self.position,
            scale: self.scale,
            size: self.size,
            dimensions: self.dimensions,
            rotation: self.rotation,
            color: self.color,
            anchor: self.anchor,
            pivot: self.pivot,
            scaling: self.scaling,
            shapes: self.shapes,
            pixel_mode: self.pixel_mode,
        }), self.depth))
    }
}

impl Shape for FrameDrawer {
    fn draw(&mut self, drawing_data: &mut DrawingData) {
        // calcualate the size of the image
        let mut size = if let Some(size) = self.size {
            size
        } else {
            Vec2::new(self.dimensions.x as f32 / self.dimensions.y as f32, 1.0) * self.scale
        };

        // transform some variable if pixel mode
        drawing_data.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;

            size *= self.dimensions.y as f32 / dims.y * 2.0;
        }); 

        let dimensions = Vec2::new(self.dimensions.x as f32, self.dimensions.y as f32);

        let mut texture_buffer = Texture2d::empty(drawing_data.display,
            self.dimensions.x,
            self.dimensions.y).unwrap();

        let mut data = DrawingData {
            frame: &mut texture_buffer,
            vertex_buffer: &mut drawing_data.vertex_buffer,
            aspect_ratio: self.dimensions.x as f32 / self.dimensions.y as f32,
            scaled_aspect_ratio: self.dimensions.x as f32 / self.dimensions.y as f32,
            window_dimensions: dimensions,
            pixel_window_dimensions: if self.pixel_mode { Some(dimensions) } else { None },
            .. *drawing_data
        };

        self.shapes.iter_mut().for_each(|(shape, _)| {
            shape.draw(&mut data);
        });

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: size.as_array(),
            rotation: Mat2::<f32>::from_radians(self.rotation).as_array(),
            anchor: (self.anchor.as_vec()).as_array(),
            pivot: (self.pivot.as_vec() / 2.0 + 0.5).as_array(),
            aspect_ratio: drawing_data.aspect_ratio,
            scaled_aspect_ratio: drawing_data.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: drawing_data.window_dimensions.as_array(),
            fill_color: self.color,
            texture_dimensions: [self.dimensions.x as f32, self.dimensions.y as f32],
            tex: &texture_buffer,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(), 
            .. Default::default()
        };

        drawing_data.frame.as_surface().draw(
            &*drawing_data.vertex_buffer, 
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), 
            drawing_data.texture,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(FrameDrawerBuilder);
rotation!(FrameDrawerBuilder);
color!(FrameDrawerBuilder);
anchor!(FrameDrawerBuilder);
pivot!(FrameDrawerBuilder);
scaling!(FrameDrawerBuilder);
depth!(FrameDrawerBuilder);
scale!(FrameDrawerBuilder);
pixel_mode!(FrameDrawerBuilder);