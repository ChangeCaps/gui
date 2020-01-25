use super::super::*;
use math::*;

pub struct Ellipse<'s> {
    position: Vec2<f32>,
    size: Vec2<f32>,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    drawing_data: &'s mut DrawingData,
}

impl<'s> Ellipse<'s> {
    pub fn new(data: &'s mut DrawingData) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            size: Vec2::new(0.2, 0.2),
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
            // calculate vertex positions
            let mut position = *vert - self.pivot.as_vec();

            position *= self.size; 
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
                shape: 1,
                index: self.drawing_data.ellipse_positions.len() as i32,
            });

        }

        let mut position = self.pivot.as_vec();

        position *= self.size; 
        position += self.position;

        if self.scaling {
            position.x /= self.drawing_data.scaled_aspect_ratio;
            self.size.x /= self.drawing_data.scaled_aspect_ratio;
        } else { 
            position.x /= self.drawing_data.aspect_ratio;
            self.size.x /= self.drawing_data.aspect_ratio;
        }

        position += self.anchor.as_vec();

        self.drawing_data.ellipse_positions.push(position.as_array());
        self.drawing_data.ellipse_sizes.push(self.size.as_array());
    }
}



position!(Ellipse);
size!(Ellipse);
color!(Ellipse);
anchor!(Ellipse);
pivot!(Ellipse);
scaling!(Ellipse);
depth!(Ellipse);
