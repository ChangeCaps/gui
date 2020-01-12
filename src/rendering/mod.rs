mod rect;
mod anchor;

pub use rect::*;
pub use anchor::*;

#[derive(Clone, Copy)]
struct Vertex {
    pub position: [f32; 2],
}

glium::implement_vertex!(Vertex, position);