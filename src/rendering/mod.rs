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

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pub position: [f32; 2],
}

#[derive(Clone, Copy, Debug)]
struct TextureVertex {
    pub position: [f32; 2],
    pub texture_coords: [f32; 2],
}


glium::implement_vertex!(Vertex, position);
glium::implement_vertex!(TextureVertex, position, texture_coords);