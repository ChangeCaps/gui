use super::super::{
    rendering::*,
    FontTexture,
};
use std::collections::HashMap;
use glium::texture::texture2d::Texture2d;

pub struct Frame<'f> { 
    pub(crate) shapes: Vec<(Box<dyn Shape>, f32)>,
    pub(crate) images: &'f HashMap<String, Texture2d>,
    pub(crate) fonts: &'f HashMap<String, FontTexture>,
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
        ImageBuilder::new(self, image.into())
    }

    pub fn text<'s, P>(&'s mut self, font: P) -> TextBuilder
        where P: Into<String>
    {
        TextBuilder::new(&mut self.shapes, font.into())
    }
}
