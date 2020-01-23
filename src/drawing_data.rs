use glium::{
    Program,
    Display,
};
use super::*;
use math::*;
use glium::texture::texture2d::Texture2d;
use std::collections::HashMap;

pub struct DrawingData<'d> {
    pub(crate) frame: &'d mut glium::texture::texture2d::Texture2d,
    pub(crate) vertex_buffer: &'d mut glium::VertexBuffer<rendering::TextureVertex>,
    pub(crate) simple_transform_fill: &'d Program,
    pub(crate) simple_transform_ellipse: &'d Program,
    pub(crate) no_transform_line: &'d Program,
    pub(crate) texture: &'d Program,
    pub(crate) text: &'d Program,
    pub(crate) display: &'d Display,
    pub(crate) window_dimensions: Vec2<f32>,
    pub(crate) pixel_window_dimensions: Option<Vec2<f32>>,
    pub(crate) aspect_ratio: f32,
    pub(crate) scaled_aspect_ratio: f32,
    pub(crate) images: &'d HashMap<String, Texture2d>,
    pub(crate) fonts: &'d HashMap<String, FontTexture>,
}
