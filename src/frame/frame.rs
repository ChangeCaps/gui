use glium::{
    Program,
    Display,
    Surface
};
use super::super::*;
use math::*;
use rendering::*;
use glium::texture::texture2d::Texture2d;

pub struct Frame<'f> {
    pub(crate) frame: &'f mut glium::framebuffer::SimpleFrameBuffer<'f>,
    pub(crate) simple_transform_fill: &'f Program,
    pub(crate) simple_transform_ellipse: &'f Program,
    pub(crate) no_transform_line: &'f Program,
    pub(crate) texture: &'f Program,
    pub(crate) text: &'f Program,
    pub(crate) display: &'f Display,
    pub(crate) window_dimensions: Vec2<f32>,
    pub(crate) pixel_window_dimensions: Option<Vec2<f32>>,
    pub(crate) aspect_ratio: f32,
    pub(crate) scaled_aspect_ratio: f32,
    pub(crate) images: &'f Vec<Texture2d>,
    pub(crate) fonts: &'f Vec<FontTexture>,
}

impl<'f> Frame<'f> {
    pub fn rect<'s>(&'s mut self) -> Rect<'s, 'f> {
        Rect::new(self)
    }

    pub fn line<'s>(&'s mut self) -> Line<'s, 'f> {
        Line::new(self)
    }

    pub fn ellipse<'s>(&'s mut self) -> Ellipse<'s, 'f> {
        Ellipse::new(self)
    }

    pub fn image<'s>(&'s mut self, image: &'s super::super::Image) -> rendering::Image<'s, 'f> {
        rendering::Image::new(self, image)
    }

    pub fn text<'s>(&'s mut self, font: &'s super::super::Font) -> rendering::Text<'s, 'f> {
        rendering::Text::new(self, font)
    }

    pub fn clear(&mut self) {
        self.frame.clear(
            None, 
            Some((0.0, 0.0, 0.0, 1.0)),
            false,
            None,
            None,
        )
    }
}
