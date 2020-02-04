use crate::*;
use math::*;
use super::*;

pub struct RectMask<'m> {
    transform: Transform,
    parent: Transform,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    drawing_data: &'m mut DrawingData,
}

impl<'m> RectMask<'m> {
    pub fn new(data: &'m mut DrawingData) -> Self {
        Self {
            transform: Transform::new(),
            parent: Transform::new(),
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            drawing_data: data,
        }
    }

    pub fn draw(self) -> Mask<'m> {
        let mut transform = self.transform.transform(self.parent);

        self.drawing_data.pixel_window_dimensions.map(|dims| {
            transform.position /= dims.y / 2.0;
            transform.size /= dims.y / 2.0;
        }); 


        transform.size.x /= if self.scaling {
            self.drawing_data.scaled_aspect_ratio
        } else {
            self.drawing_data.aspect_ratio
        };

        transform.position += self.anchor.as_vec() * 
            if let Some(size) = self.drawing_data.pixel_window_dimensions {
                size / 2.0
            } else {
                Vec2::new(1.0, 1.0)
            };
    
        // this could also be the length of either rect_mask_sizes or rect_mask_rotations
        // we just need the index
        let index = self.drawing_data.mask_shapes.len();

        self.drawing_data.mask_shapes.push([0, self.drawing_data.rect_mask_positions.len() as i32]);
        self.drawing_data.rect_mask_positions.push(transform.position.as_array());
        self.drawing_data.rect_mask_sizes.push(transform.size.as_array());
        self.drawing_data.rect_mask_rotations.push(Mat2::<f32>::from_radians(transform.rotation).as_array());

        Mask {
            masks: (index as i32, 1),
            drawing_data: self.drawing_data,
        }
    }
}

transform!(RectMask);
color!(RectMask);
anchor!(RectMask);
pivot!(RectMask);
scaling!(RectMask);
