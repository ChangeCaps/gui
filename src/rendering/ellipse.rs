use super::super::*;
use math::*;

pub struct Ellipse<'s> {
    transform: Transform,
    parent: Transform,
    masks: (i32, i32),
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    drawing_data: &'s mut DrawingData,
}

impl<'s> Ellipse<'s> {
    pub fn new(data: &'s mut DrawingData, masks: (i32, i32)) -> Self {
        Self {
            transform: Transform::new(),
            parent: Transform::new(),
            masks,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            drawing_data: data,
        }
    }

    pub fn draw(&mut self) -> Transform {
        self.transform = self.transform.transform(self.parent);

        self.transform.position /= self.drawing_data.frame_size.y / 2.0;
        self.transform.size /= self.drawing_data.frame_size.y / 2.0;

        for vert in &RECT_VERTS {
            // calculate vertex positions
            let mut position = *vert - self.pivot.as_vec(); 

            position = position.transform(self.transform);

            if self.scaling {
                position.x /= self.drawing_data.scaled_aspect_ratio;
            } else { 
                position.x /= self.drawing_data.aspect_ratio;
            }

            position += self.anchor.as_vec(); 

            self.drawing_data.verts.push(super::Vertex {
                position: position.as_array(),
                texture_coords: vert.as_array(),
                color: self.color,
                depth: self.depth,
                shape: 1,
                shape_index: 0,
                mask_length: self.masks.1,
                mask_index: self.masks.0,
            });

        }

        self.transform
    }
}



transform!(Ellipse);
color!(Ellipse);
anchor!(Ellipse);
pivot!(Ellipse);
scaling!(Ellipse);
depth!(Ellipse);
