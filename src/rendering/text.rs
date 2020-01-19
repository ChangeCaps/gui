use super::super::*;
use super::TextureVertex;
use math::*;
use glium;
use glium::Surface;

pub struct TextBuilder<'s> {
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
    shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>,
}

pub struct Text {
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
}

impl<'s> TextBuilder<'s> {
    pub fn new(shape_vec: &'s mut Vec<(Box<dyn super::Shape>, f32)>, font: String) -> Self {
        Self {
            font,
            position: Vec2::new(0.0, 0.0),
            scale: 0.2,
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            pivot: Anchor::Middle,
            scaling: false,
            depth: 0.0,
            text: String::new(),
            shape_vec
        }
    }

    // set the text
    pub fn text<T>(mut self, text: T) -> Self 
        where T: Into<String>
    {
        self.text = text.into();
        self
    }

    pub fn draw(self) {
        self.shape_vec.push((Box::new(Text {
            position: self.position,
            scale: self.scale,
            rotation: self.rotation,
            color: self.color,
            anchor: self.anchor,
            pivot: self.pivot,
            scaling: self.scaling,
            depth: self.depth,
            text: self.text,
            font: self.font,
        }), self.depth))
    }
}

impl super::Shape for Text {
    fn draw(&mut self, frame: &mut Frame) {
        // don't draw if there is no text
        if self.text.len() == 0 {
            return;
        }

        let font = frame.fonts.get(&self.font).unwrap();

        frame.pixel_window_dimensions.map(|dims| {
            self.position /= dims.y / 2.0;
            self.scale /= dims.y / 2.0;
        }); 

        let mut vertex_buffer_data = Vec::with_capacity(self.text.len() * 4 * 4);
        let mut index_buffer_data = Vec::with_capacity(self.text.len() * 6);

        let mut total_text_width = 0.0;

        let mut height = 0.0;

        // calc verts & indecies
        for character in self.text.chars() {
            let infos = match font.character_infos.get(&character) {
                Some(infos) => infos,
                None => continue,
            };

            // adding the quad in the index buffer
            {
                let first_vertex_offset = vertex_buffer_data.len() as u16;
                index_buffer_data.push(first_vertex_offset);
                index_buffer_data.push(first_vertex_offset + 1);
                index_buffer_data.push(first_vertex_offset + 2);
                index_buffer_data.push(first_vertex_offset + 2);
                index_buffer_data.push(first_vertex_offset + 1);
                index_buffer_data.push(first_vertex_offset + 3);
            }

            // add to total width
            total_text_width += infos.left_padding;
            
            // calculating coords
            let left_coord = total_text_width;
            let right_coord = left_coord + infos.size.0;
            let top_coord = infos.height_over_line;
            let bottom_coord = infos.height_over_line - infos.size.1;
            
            total_text_width += infos.size.0 + infos.right_padding;

            // top-left vertex
            vertex_buffer_data.push(TextureVertex {
                position: [left_coord, top_coord],
                texture_coords: [infos.tex_coords.0, infos.tex_coords.1],
            });

            // top-right vertex
            vertex_buffer_data.push(TextureVertex {
                position: [right_coord, top_coord],
                texture_coords: [infos.tex_coords.0 + infos.tex_size.0, infos.tex_coords.1],
            });

            // bottom-left vertex
            vertex_buffer_data.push(TextureVertex {
                position: [left_coord, bottom_coord],
                texture_coords: [infos.tex_coords.0, infos.tex_coords.1 + infos.tex_size.1],
            });

            // bottom-right vertex
            vertex_buffer_data.push(TextureVertex {
                position: [right_coord, bottom_coord],
                texture_coords: [
                    infos.tex_coords.0 + infos.tex_size.0,
                    infos.tex_coords.1 + infos.tex_size.1
                ],
            });

            if infos.height_over_line > height {
                height = infos.height_over_line;
            }
        }

        let vertex_buffer = glium::VertexBuffer::new(frame.display, &vertex_buffer_data)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(frame.display, glium::index::PrimitiveType::TrianglesList, &index_buffer_data)
            .expect("failed to create index buffer");   

        // calculate pivot
        let mut pivot = self.pivot.as_vec()/2.0 + 0.5;
        pivot.x *= total_text_width;
        pivot.y *= height;

        let uniforms = uniform!{
            pos: self.position.as_array(),
            size: [self.scale/height, self.scale/height],
            rotation: Mat2::<f32>::from_degrees(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            pivot: pivot.as_array(),
            aspect_ratio: frame.aspect_ratio,
            scaled_aspect_ratio: frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: frame.window_dimensions.as_array(),
            fill_color: self.color,
            tex: glium::uniforms::Sampler(&font.texture, Default::default())
        };

        // enable alpha blending
        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            depth: glium::Depth {
                test: glium::DepthTest::IfMoreOrEqual,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // draw
        frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            frame.text,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(TextBuilder);
rotation!(TextBuilder);
color!(TextBuilder);
anchor!(TextBuilder);
pivot!(TextBuilder);
scaling!(TextBuilder);
scale!(TextBuilder);
depth!(TextBuilder);