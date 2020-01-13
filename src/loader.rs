use super::Image;
use super::math::*;
use std::io::Read;
use glium::texture::CompressedSrgbTexture2d;

pub struct Loader<'s> {
    pub(crate) display: &'s glium::Display,
    pub(crate) images: &'s mut Vec<CompressedSrgbTexture2d>,
}

impl<'s> Loader<'s> {
    pub fn load_image(&mut self, path: &'static str, format: image::ImageFormat) -> Image {
        let mut buf = Vec::new();

        std::fs::File::open(path)
            .expect(format!("GUI::IMAGE Failed to open image located at path: {}", path).as_str()).read_to_end(&mut buf)
            .expect(format!("GUI::IMAGE Failed to read image located at path: {}", path).as_str());

        let image = image::load(std::io::Cursor::new(buf), format)
            .expect(format!("GUI::IMAGE Failed to load image at path: {}", path).as_str()).to_rgba();

        let image_dimensions = image.dimensions();

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = CompressedSrgbTexture2d::new(self.display, image)
            .expect("GUI::IMAGE Failed to create texture buffer");

        let index = self.images.len();

        self.images.push(texture);

        Image {
            index,
            dimensions: Vec2::new(image_dimensions.0 as f32, image_dimensions.1 as f32)
        }
    }
}
