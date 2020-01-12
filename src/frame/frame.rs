use glium::{
    Program,
    Display,
    Surface
};
use super::super::*;
use math::*;
use rendering::*;

pub struct Frame<'f> {
    pub(crate) frame: &'f mut glium::Frame,
    pub(crate) simple_transform_fill: &'f Program,
    pub(crate) simple_transform_ellipse: &'f Program,
    pub(crate) no_transform_line: &'f Program,
    pub(crate) display: &'f Display,
    pub(crate) window_dimensions: Vec2<f32>,
    pub(crate) aspect_ratio: f32,
    pub(crate) scaled_aspect_ratio: f32,
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