use super::*;
use math::*;
use std::collections::HashMap;

#[derive(Default, Clone)]
pub struct DrawingData {
    pub(crate) verts: Vec<rendering::Vertex>, 

    // lines
    pub(crate) line_points: Vec<[f32; 4]>,
    pub(crate) line_widths: Vec<f32>,

    // masks
    pub(crate) mask_shapes: Vec<[i32; 2]>,

    // rect mask
    pub(crate) rect_mask_positions: Vec<[f32; 2]>,
    pub(crate) rect_mask_sizes: Vec<[f32; 2]>,
    pub(crate) rect_mask_rotations: Vec<[[f32; 2]; 2]>,

    // window information
    pub(crate) frame_size: Vec2<f32>,
    pub(crate) aspect_ratio: f32,
    pub(crate) scaled_aspect_ratio: f32,

    // text
    pub(crate) font_indecies: HashMap<String, usize>,
    pub(crate) font_dimensions: Vec<Vec2<u32>>,
    pub(crate) font_positions: Vec<Vec2<f32>>,
    pub(crate) font_character_infos: Vec<HashMap<char, CharacterInfos>>,
    pub(crate) font_atlas_dimensions: Vec2<f32>,
    
    // images
    pub(crate) image_indecies: HashMap<String, usize>,
    pub(crate) image_dimensions: Vec<Vec2<u32>>,
    pub(crate) image_positions: Vec<Vec2<f32>>,
    pub(crate) image_atlas_dimensions: Vec2<f32>,
}
