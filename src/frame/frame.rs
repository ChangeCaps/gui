use glium::{
    Program,
    Display,
};
use super::super::*;
use rendering::*;

pub struct Frame<'f> {
    pub(crate) frame: &'f mut glium::Frame,
    pub(crate) fill_polygon: &'f Program,
    pub(crate) display: &'f Display,
    pub(crate) scale_aspect_ratio: bool,
    pub(crate) window_dimensions: [f32; 2],
}

impl<'f> Frame<'f> {
    pub fn rect<'s>(&'s mut self) -> Rect<'s, 'f> {
        Rect::new(self)
    }
}