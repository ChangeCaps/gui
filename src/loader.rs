use super::Image;
use super::Font;
use super::FontTexture;
use super::TextInput;
use super::math::*;
use std::io::Read;
use glium::texture::texture2d::Texture2d;

pub struct Loader<'s> {
    pub(crate) display: &'s glium::Display,
    pub(crate) images: &'s mut Vec<Texture2d>,
    pub(crate) fonts: &'s mut Vec<FontTexture>,
    pub(crate) text_inputs: &'s mut Vec<std::rc::Rc<std::cell::RefCell<(String, bool)>>>,
}

impl<'s> Loader<'s> {
    pub fn load_font(&mut self, path: &'static str, font_size: u32) -> Font {
        let file = std::fs::File::open(path)
            .expect(format!("GUI::TEXT Failed to open font located at path: {}", path).as_str());

        let font_texture = FontTexture::new(self.display, file, font_size, FontTexture::ascii_character_list())
            .expect(format!("GUI::TEXT Failed to load font located at path: {}", path).as_str());

        let index = self.fonts.len();
        self.fonts.push(font_texture);

        Font {
            index,
        }
    }

    pub fn load_image(&mut self, path: &'static str, format: image::ImageFormat) -> Image {
        let mut buf = Vec::new();

        std::fs::File::open(path)
            .expect(format!("GUI::IMAGE Failed to open image located at path: {}", path).as_str()).read_to_end(&mut buf)
            .expect(format!("GUI::IMAGE Failed to read image located at path: {}", path).as_str());

        let image = image::load(std::io::Cursor::new(buf), format)
            .expect(format!("GUI::IMAGE Failed to load image at path: {}", path).as_str()).to_rgba();

        let image_dimensions = image.dimensions();

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let texture = Texture2d::new(self.display, image)
            .expect("GUI::IMAGE Failed to create texture buffer");

        let index = self.images.len();

        self.images.push(texture);

        Image {
            index,
            dimensions: Vec2::new(image_dimensions.0 as f32, image_dimensions.1 as f32)
        }
    }

    pub fn text_input(&mut self) -> TextInput {
        let cell = std::rc::Rc::new(std::cell::RefCell::new((String::new(), false)));

        self.text_inputs.push(cell.clone());

        TextInput {
            text: cell,
        }
    }
}
