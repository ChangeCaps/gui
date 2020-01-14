use super::super::*;
use super::TextureVertex;
use math::*;
use glium;
use glium::Surface;

pub struct Text<'s, 'f> {
    font: &'s super::super::Font,
    position: Vec2<f32>,
    rotation: f32,
    color: [f32; 4],
    frame: &'s mut Frame<'f>,
    anchor: Anchor,
    scaling: bool,
    text: String,
}

impl<'s, 'f> Text<'s, 'f> {
    pub fn new(frame: &'s mut Frame<'f>, font: &'s super::super::Font) -> Self {
        Self {
            font,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            color: color::rgb(1.0, 1.0, 1.0),
            anchor: Anchor::Middle,
            scaling: false,
            frame,
            text: String::new(),
        }
    }

    pub fn text<T>(mut self, text: T) -> Self 
        where T: Into<String>
    {
        self.text = text.into();
        self
    }

    pub fn draw(mut self) {
        if self.text.len() == 0 {
            return;
        }

        let mut vertex_buffer_data = Vec::with_capacity(self.text.len() * 4 * 4);
        let mut index_buffer_data = Vec::with_capacity(self.text.len() * 6);

        let mut total_text_width = 0.0;

        for character in self.text.chars() {
            let infos = match self.frame.fonts[self.font.index].character_infos.get(&character) {
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

            //
            total_text_width += infos.left_padding;

            // calculating coords
            let left_coord = total_text_width;
            let right_coord = left_coord + infos.size.0;
            let top_coord = infos.height_over_line;
            let bottom_coord = infos.height_over_line - infos.size.1;

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
        }

        let vertex_buffer = glium::VertexBuffer::new(self.frame.display, &vertex_buffer_data)
            .expect("failed to create vertex buffer");

        let index_buffer = glium::IndexBuffer::new(self.frame.display, glium::index::PrimitiveType::TrianglesList, &index_buffer_data)
            .expect("failed to create index buffer");   

        let mut height = 0.0;

        for (_, info) in &self.frame.fonts[self.font.index].character_infos {
            if info.height_over_line > height {
                height = info.height_over_line;
            }
        }

        let mut position = self.position;
        position.x -= total_text_width;
        position.y -= height;

        let uniforms = uniform!{
            pos: position.as_array(),
            size: [1.0f32, 1.0f32],
            rotation: Mat2::<f32>::from_degrees(self.rotation).as_array(),
            anchor: self.anchor.as_vec().as_array(),
            aspect_ratio: self.frame.aspect_ratio,
            scaled_aspect_ratio: self.frame.scaled_aspect_ratio,
            scale_aspect_ratio: self.scaling,
            window_dimensions: self.frame.window_dimensions.as_array(),
            fill_color: self.color,
            tex: glium::uniforms::Sampler(&self.frame.fonts[self.font.index].texture, Default::default())
        };

        let draw_params = glium::DrawParameters {
            blend: glium::Blend::alpha_blending(),
            .. Default::default()
        };

        self.frame.frame.draw(
            &vertex_buffer, 
            &index_buffer, 
            self.frame.text,
            &uniforms,
            &draw_params,
        ).expect("failed to draw rect");
    }
}

position!(Text);
rotation!(Text);
color!(Text);
anchor!(Text);
scaling!(Text);