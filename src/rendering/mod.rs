mod rect;
mod anchor;
mod line;

pub use rect::*;
pub use anchor::*;
pub use line::*;

#[derive(Clone, Copy, Debug)]
struct Vertex {
    pub position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);