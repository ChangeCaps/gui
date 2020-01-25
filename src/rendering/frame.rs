use crate::{
    rendering::*,
    math::*,
};
use glium::{
    texture::texture2d::Texture2d,
    Surface,
};
use super::super::*;

pub trait Canvas<'f> {
    fn drawing_data(&mut self) -> &mut DrawingData;

    fn rect(&mut self) -> Rect {
        Rect::new(self.drawing_data())
    }

    fn line(&mut self) -> Line {
        Line::new(self.drawing_data())
    }

    fn ellipse(&mut self) -> Ellipse {
        Ellipse::new(self.drawing_data())
    }

    /*
    fn image<P>(&mut self, image: P) -> ImageBuilder
        where P: Into<String>
    {
        ImageBuilder::new(self.get_shapes(), image.into())
    }

    fn text<P>(&mut self, font: P) -> TextBuilder
        where P: Into<String>
    {
        TextBuilder::new(self.get_shapes(), font.into())
    }

    fn new_frame(&mut self) -> Frame {
        Frame {
            shapes: Vec::new(),
        }
    }*/
}

pub struct Frame<'f> { 
    pub(crate) drawing_data: &'f mut DrawingData,
}

impl<'f> Canvas<'f> for Frame<'f> {
    fn drawing_data(&mut self) -> &mut DrawingData {
        self.drawing_data
    }
}
