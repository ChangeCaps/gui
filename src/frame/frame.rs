use glium::{
    Program,
    Display,
    Surface
};
use super::super::*;
use math::*;
use rendering::*;
use glium::texture::texture2d::Texture2d;
use std::collections::HashMap;

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
    pub(crate) images: &'f HashMap<String, Texture2d>,
    pub(crate) fonts: &'f HashMap<String, FontTexture>,
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

    pub fn image<'s, P>(&'s mut self, image: P) -> rendering::Image<'s, 'f> 
        where P: Into<String>
    {
        rendering::Image::new(self, image.into())
    }

    pub fn text<'s, P>(&'s mut self, font: P) -> rendering::Text<'s, 'f> 
        where P: Into<String>
    {
        rendering::Text::new(self, font.into())
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
