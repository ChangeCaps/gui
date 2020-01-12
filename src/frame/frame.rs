use glium::{
    Program,
    Display,
};
use super::super::*;
use rendering::*;

pub struct Frame<'f> {
    pub(crate) frame: &'f mut glium::Frame,
    pub(crate) simple_transform_fill: &'f Program,
    pub(crate) no_transform_line: &'f Program,
    pub(crate) display: &'f Display,
    pub(crate) window_dimensions: [f32; 2],
}

impl<'f> Frame<'f> {
    pub fn rect<'s>(&'s mut self) -> Rect<'s, 'f> {
        Rect::new(self)
    }

    pub fn line<'s>(&'s mut self) -> Line<'s, 'f> {
        Line::new(self)
    }
}