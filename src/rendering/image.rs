use super::super::*;
use math::*;

pub struct Image<'s> {
    image: String,
    transform: Transform,     
    parent: Transform,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    drawing_data: &'s mut DrawingData,
}

impl<'s> Image<'s> {
    pub fn new(data: &'s mut DrawingData, image: String) -> Self {
        let index = *data.image_indecies.get(&image).unwrap();

        let dimensions = data.image_dimensions[index];
        let dimensions = Vec2::new(dimensions.x as f32, dimensions.y as f32);

        Self {
            image,
            transform: Transform {
                size: if data.pixel_window_dimensions.is_some() {
                    dimensions
                } else {
                    Vec2::new(dimensions.x/dimensions.y, 1.0)
                },
                .. Default::default()
            },
            parent: Transform::new(),
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            drawing_data: data,
        }
    } 

    pub fn draw(mut self) -> Transform {
        let index = *self.drawing_data.image_indecies.get(&self.image).unwrap();

        let image_position = self.drawing_data.image_positions[index];
        
        let dimensions = self.drawing_data.image_dimensions[index];
        let dimensions = Vec2::new(dimensions.x as f32, dimensions.y as f32);

        self.transform = self.transform.transform(self.parent);

        self.drawing_data.pixel_window_dimensions.map(|dims| {
            self.transform.position /= dims.y / 2.0;
            self.transform.size /= dims.y / 2.0;
        }); 

        for vert in &RECT_VERTS {
            let mut position = *vert - self.pivot.as_vec() / 2.0;

            position = position.transform(self.transform);
            
            if self.scaling {
                position.x /= self.drawing_data.scaled_aspect_ratio;
            } else { 
                position.x /= self.drawing_data.aspect_ratio;
            }

            position += self.anchor.as_vec();

            self.drawing_data.verts.push(super::Vertex {
                position: position.as_array(),
                texture_coords: ((*vert + 0.5) * dimensions / 
                                self.drawing_data.image_atlas_dimensions + image_position).as_array(),
                color: self.color,
                depth: self.depth,
                shape: 3,
                shape_index: index as i32,
                mask_length: 0,
                mask_index: 0,
            });
        }

        self.transform
    }
}

transform!(Image);
color!(Image);
anchor!(Image);
pivot!(Image);
scaling!(Image);
depth!(Image);
