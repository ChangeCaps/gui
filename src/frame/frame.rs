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
    pub(crate) shapes: Vec<(Box<dyn Shape>, f32)>,
}

impl<'f> Frame<'f> {
    pub fn rect<'s>(&'s mut self) -> RectBuilder {
        RectBuilder::new(&mut self.shapes)
    }

    pub fn line<'s>(&'s mut self) -> LineBuilder {
        LineBuilder::new(&mut self.shapes)
    }

    pub fn ellipse<'s>(&'s mut self) -> EllipseBuilder {
        EllipseBuilder::new(&mut self.shapes)
    }

    pub fn image<'s, P>(&'s mut self, image: P) -> ImageBuilder
        where P: Into<String>
    {
        rendering::ImageBuilder::new(self, image.into())
    }

    pub fn text<'s, P>(&'s mut self, font: P) -> TextBuilder
        where P: Into<String>
    {
        rendering::TextBuilder::new(&mut self.shapes, font.into())
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
