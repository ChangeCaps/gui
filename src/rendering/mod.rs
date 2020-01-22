mod rect;
mod anchor;
mod line;
mod ellipse;
mod image;
mod text;
mod traits;

pub use rect::*;
pub use anchor::*;
pub use line::*;
pub use ellipse::*;
pub use self::image::*;
pub use text::*;

use crate::drawing_data::*;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pub position: [f32; 2],
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct TextureVertex {
    pub position: [f32; 2],
    pub texture_coords: [f32; 2],
}

pub trait Shape {
    fn draw<'f>(&mut self, drawing_data: &mut DrawingData<'f>);
}

glium::implement_vertex!(Vertex, position);
glium::implement_vertex!(TextureVertex, position, texture_coords);
