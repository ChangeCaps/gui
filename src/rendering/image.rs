use super::super::*;
use math::*;

pub struct Image<'s> {
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
    drawing_data: &'s mut DrawingData,
}

impl<'s> Image<'s> {
    pub fn new(data: &'s mut DrawingData, image: String) -> Self {
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
            drawing_data: data,
        }
    }

    pub fn size(mut self, size: Vec2<f32>) -> Self {
        self.size = Some(size);
        self
    }

    pub fn draw(mut self) {
        let index = *self.drawing_data.image_indecies.get(&self.image).unwrap();

        let dimensions = self.drawing_data.image_dimensions[index];

        let ratio = dimensions.x / dimensions.y;

        let mut size = if let Some(size) = self.size {
            size
        } else {
            Vec2::new(ratio, 1.0)
        };

        self.drawing_data.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            size /= dims.y / 2.0;
        }); 

        for vert in &RECT_VERTS {
            let mut position = *vert - self.pivot.as_vec() / 2.0;

            position *= size; 
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
                shape: 3,
                index: index as i32,
            });
        }


        self.drawing_data.rects += 1;    
    }
}

position!(Image);
rotation!(Image);
color!(Image);
anchor!(Image);
pivot!(Image);
scaling!(Image);
depth!(Image);
scale!(Image);
