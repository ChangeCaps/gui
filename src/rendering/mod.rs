mod rect;
mod anchor;
mod line;
mod ellipse;

pub use rect::*;
pub use anchor::*;
pub use line::*;
pub use ellipse::*;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pub position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);