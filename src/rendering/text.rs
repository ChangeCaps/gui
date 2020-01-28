use super::super::*;
use math::*;
use glium;
use glium::Surface;

pub struct Text<'s> {
    font: String,
    position: Vec2<f32>,
    scale: f32,
    rotation: f32,
    color: [f32; 4],
    anchor: Anchor,
    pivot: Anchor,
    scaling: bool,
    depth: f32,
    text: String,
    drawing_data: &'s mut DrawingData,
}

impl<'s> Text<'s> {
    pub fn new(data: &'s mut DrawingData, font: String) -> Self {
        Self {
            font,
            position: Vec2::new(0.0, 0.0),
            scale: 0.2,
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            text: String::new(),
            depth: 0.0,
            drawing_data: data,
        }
    }

    // set the text
    pub fn text<T>(mut self, text: T) -> Self 
        where T: Into<String>
    {
        self.text = text.into();
        self
    }

    pub fn draw(&mut self) {
        // don't draw if there is no text
        if self.text.len() == 0 {
            return;
        }

        self.drawing_data.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            self.scale /= dims.y / 2.0;
        }); 

        let index = self.drawing_data.font_indecies.get(&self.font).unwrap();
        let font = &self.drawing_data.font_character_infos[*index];
        let font_position = self.drawing_data.font_positions[*index];
        let dimensions = self.drawing_data.font_dimensions[*index];
        let dimensions = Vec2::new(dimensions.x as f32, dimensions.y as f32);

        let mut verts = Vec::with_capacity(self.text.len() * 6);
        let mut total_text_width = 0.0;
        let mut height = 0.0;

        // calc verts & indecies
        for character in self.text.chars() {
            let infos = match font.get(&character) {
                Some(infos) => infos,
                None => continue,
            };

            // add to total width
            total_text_width += infos.left_padding;
            
            // calculating coords
            let left_coord = total_text_width;
            let right_coord = left_coord + infos.size.0;
            let top_coord = infos.height_over_line;
            let bottom_coord = infos.height_over_line - infos.size.1;
            
            total_text_width += infos.size.0 + infos.right_padding;

            // top-left vertex
            verts.push((
                Vec2::new(left_coord, top_coord),
                Vec2::new(infos.tex_coords.0, infos.tex_coords.1),
            ));

            // top-right vertex
            verts.push((
                Vec2::new(right_coord, top_coord),
                Vec2::new(infos.tex_coords.0 + infos.tex_size.0, infos.tex_coords.1),
            ));

            // bottom-left vertex
            verts.push((
                Vec2::new(left_coord, bottom_coord),
                Vec2::new(infos.tex_coords.0, infos.tex_coords.1 + infos.tex_size.1),
            ));
            


            // top-right vertex
            verts.push((
                Vec2::new(right_coord, top_coord),
                Vec2::new(infos.tex_coords.0 + infos.tex_size.0, infos.tex_coords.1),
            ));

            // bottom-left vertex
            verts.push((
                Vec2::new(left_coord, bottom_coord),
                Vec2::new(infos.tex_coords.0, infos.tex_coords.1 + infos.tex_size.1),
            ));

            // bottom-right vertex
            verts.push((
                Vec2::new(right_coord, bottom_coord),
                Vec2::new(
                    infos.tex_coords.0 + infos.tex_size.0,
                    infos.tex_coords.1 + infos.tex_size.1
                ),
            ));

            if infos.height_over_line > height {
                height = infos.height_over_line;
            }
        }

        // calculate pivot
        let mut pivot = self.pivot.as_vec();
        pivot.x *= total_text_width;
        pivot.y += 0.5;
        pivot.y *= height;


        for vert in verts {
            let mut position = vert.0 - pivot;

            position *= self.scale; 
            position *= Mat2::<f32>::from_radians(self.rotation);
            position += self.position;

            if self.scaling {
                position.x /= self.drawing_data.scaled_aspect_ratio;
            } else { 
                position.x /= self.drawing_data.aspect_ratio;
            }

            position += self.anchor.as_vec();

            self.drawing_data.verts.push(
                super::Vertex {
                    position: position.as_array(),
                    texture_coords: (vert.1 * dimensions / self.drawing_data.font_atlas_dimensions).as_array(),
                    color: self.color,
                    depth: self.depth,
                    shape: 4,
                    shape_index: *index as i32,
                    mask_length: 0,
                    mask_index: 0,
                }
            );
        }
    }
}

position!(Text);
rotation!(Text);
color!(Text);
anchor!(Text);
pivot!(Text);
scaling!(Text);
scale!(Text);
depth!(Text);
