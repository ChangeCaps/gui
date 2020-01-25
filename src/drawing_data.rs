use super::*;
use math::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct DrawingData {
    pub(crate) verts: Vec<rendering::Vertex>, 
    pub(crate) rects: i32, 
    pub(crate) ellipse_positions: Vec<[f32; 2]>,
    pub(crate) ellipse_sizes: Vec<[f32; 2]>,
    pub(crate) line_points: Vec<[f32; 4]>,
    pub(crate) line_widths: Vec<f32>,
    pub(crate) pixel_window_dimensions: Option<Vec2<f32>>,
    pub(crate) aspect_ratio: f32,
    pub(crate) scaled_aspect_ratio: f32,
    pub(crate) image_indecies: HashMap<String, usize>,
    pub(crate) font_indecies: HashMap<String, usize>,
    pub(crate) image_dimensions: Vec<Vec2<f32>>,
}
