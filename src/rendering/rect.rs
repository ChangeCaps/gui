use super::super::*;
use math::*;

pub struct Rect<'s> {
    position: Vec2<f32>,
    size: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    drawing_data: &'s mut DrawingData,
}

impl<'s> Rect<'s> {
    pub fn new(data: &'s mut DrawingData) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.2, 0.2),
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            drawing_data: data,
        }
    }
    
    pub fn draw(&mut self) {
        self.drawing_data.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            self.size /= dims.y / 2.0;
        }); 

        for vert in &RECT_VERTS {
            let mut position = *vert - self.pivot.as_vec() / 2.0;

            position *= self.size; 
            position *= Mat2::<f32>::from_radians(self.rotation);
            position += self.position;

            if self.scaling {
                position.x /= self.drawing_data.scaled_aspect_ratio;
            } else { 
                position.x /= self.drawing_data.aspect_ratio;
            }

            position += self.anchor.as_vec();

            self.drawing_data.verts.push(super::Vertex {
                position: position.as_array(),
                texture_coords: (*vert + 0.5).as_array(),
                color: self.color,
                depth: self.depth,
                shape: 0,
                index: self.drawing_data.rects,
            });
        }


        self.drawing_data.rects += 1;
    }
}


position!(Rect);
size!(Rect);
rotation!(Rect);
color!(Rect);
anchor!(Rect);
pivot!(Rect);
scaling!(Rect);
depth!(Rect);
