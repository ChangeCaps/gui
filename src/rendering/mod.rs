//! Rendering in fumarole is handled by drawing 'Shapes' to the window.

mod rect;
mod anchor;
mod line;
mod ellipse;
mod image;
mod text;
mod traits;
mod frame;
mod mask;

pub use rect::*;
pub use anchor::*;
pub use line::*;
pub use ellipse::*;
pub use self::image::*;
pub use text::*;
pub use frame::*;
pub use mask::*;

use crate::drawing_data::*;
use crate::math::*;

pub(crate) const RECT_VERTS: [Vec2<f32>; 6] = [
    Vec2 { x: -0.5, y: -0.5 },
    Vec2 { x:  0.5, y: -0.5 },
    Vec2 { x: -0.5, y:  0.5 }, 

    Vec2 { x:  0.5, y: -0.5 },
    Vec2 { x: -0.5, y:  0.5 }, 
    Vec2 { x:  0.5, y:  0.5 },
];

#[derive(Clone, Copy, Debug)]
pub(crate) struct Vertex {
    pub position: [f32; 2],
    pub texture_coords: [f32; 2],
    pub color: [f32; 4],
    pub depth: f32,
    pub shape: i32,
    pub shape_index: i32,
    pub mask_length: i32,
    pub mask_index: i32,
}

glium::implement_vertex!(Vertex, 
                         position, 
                         texture_coords, 
                         color,
                         shape, 
                         shape_index,
                         mask_length,
                         mask_index);
