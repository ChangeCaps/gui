use super::math::*;

pub struct Image {
    pub(crate) texture: glium::texture::CompressedSrgbTexture2d,
    pub(crate) dimensions: Vec2<f32>,
}

impl Image {
    pub fn dimensions(&self) -> Vec2<f32> {
        self.dimensions
    }
}