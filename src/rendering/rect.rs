use super::super::*;
use math::*;
use glium;
use glium::Surface;

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
        }), self.depth))
    }
}

impl super::Shape for Rect {
    fn draw(&mut self, drawing_data: &mut DrawingData) {
        drawing_data.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            self.size /= dims.y / 2.0;
        }); 

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: self.size.as_array(),
            rotation: Mat2::<f32>::from_radians(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            pivot: (self.pivot.as_vec() / 2.0 + 0.5).as_array(),
            aspect_ratio: drawing_data.aspect_ratio,
            scaled_aspect_ratio: drawing_data.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: drawing_data.window_dimensions.as_array(),
            fill_color: self.color,
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(), 
            .. Default::default()
        };

        drawing_data.frame.as_surface().draw(
            &*drawing_data.vertex_buffer, 
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList), 
            drawing_data.simple_transform_fill,
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
