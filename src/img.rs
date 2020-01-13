use super::math::*;

pub struct Image {
    pub(crate) index: usize,
    pub(crate) dimensions: Vec2<f32>,
}

impl Image {
    pub fn dimensions(&self) -> Vec2<f32> {
        self.dimensions
    }
}
