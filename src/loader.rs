//! Used for loading images and fonts. When loading an asset, you provide an identification string
//! for the purposes of drawing later in the program. It is usually the path to the asset but in
//! some cases just an arbitrary string.

use super::FontTexture;
use super::TextInput;
use super::math::*;
use super::font::CharacterInfos;
use std::io::Read;
use glium::texture::RawImage2d;
use std::collections::HashMap;

pub struct Loader<'s> {
    pub(crate) display: &'s glium::Display,
    pub(crate) image_indecies: &'s mut HashMap<String, usize>,
    pub(crate) font_indecies: &'s mut HashMap<String, usize>,
    pub(crate) images: Vec<RawImage2d<'s, u8>>,
    pub(crate) fonts: Vec<RawImage2d<'s, f32>>,
    pub(crate) image_dimensions: &'s mut Vec<Vec2<u32>>,
    pub(crate) font_dimensions: &'s mut Vec<Vec2<u32>>,
    pub(crate) font_character_infos: &'s mut Vec<HashMap<char, CharacterInfos>>,
    pub(crate) text_inputs: &'s mut Vec<std::rc::Rc<std::cell::RefCell<(String, bool)>>>,
}

impl<'s> Loader<'s> {
    /// Loads font from a path and a font size
    pub fn load_font<P>(&mut self, path: P, font_size: u32) 
        where P: Into<String> + AsRef<std::path::Path> + std::fmt::Display + Copy
    {
        let file = std::fs::File::open(path)
            .expect(format!("GUI::TEXT Failed to open font located at path: {}", path).as_str());

        let (font_texture, character_infos) = FontTexture::new(self.display, 
                                                               file, 
                                                               font_size, 
                                                               FontTexture::ascii_character_list())
            .expect(format!("GUI::TEXT Failed to load font located at path: {}", path).as_str());

        let font_dimensions = Vec2::new(font_texture.width, font_texture.height);

        self.font_dimensions.push(font_dimensions);
        self.font_indecies.insert(path.into(), self.fonts.len());
        self.fonts.push(font_texture);
        self.font_character_infos.push(character_infos);
    }

    /// Loads an image from a path
    pub fn load_image<P>(&mut self, path: P, format: image::ImageFormat) -> Vec2<u32>
        where P: Into<String> + AsRef<std::path::Path> + std::fmt::Display + Copy
    {
        let mut buf = Vec::new();

        std::fs::File::open(path)
            .expect(format!("GUI::IMAGE Failed to open image located at path: {}", path).as_str())
            .read_to_end(&mut buf)
            .expect(format!("GUI::IMAGE Failed to read image located at path: {}", path).as_str());

        let image = image::load(std::io::Cursor::new(buf), format)
            .expect(format!("GUI::IMAGE Failed to load image at path: {}", path).as_str()).to_rgba();

        let image_dimensions = image.dimensions();

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let image_dimensions = Vec2::new(image_dimensions.0, image_dimensions.1);

        self.image_dimensions.push(image_dimensions);
        self.image_indecies.insert(path.into(), self.images.len());
        self.images.push(image);

        image_dimensions
    }

    /// Loads an image from raw data
    pub fn load_image_from_raw<I>(&mut self, data: &[u8], format: image::ImageFormat, ident: I) -> Vec2<u32> 
        where I: Into<String> + AsRef<std::path::Path> + std::fmt::Display + Copy
    {
        let image = image::load(std::io::Cursor::new(data), format)
            .expect(format!("GUI::IMAGE Failed to load image: {}", ident).as_str()).to_rgba();

        let image_dimensions = image.dimensions();

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

        let image_dimensions = Vec2::new(image_dimensions.0, image_dimensions.1);

        self.image_dimensions.push(image_dimensions);
        self.image_indecies.insert(ident.into(), self.images.len());
        self.images.push(image);

        image_dimensions
    }

    /// Crates a text input
    pub fn text_input(&mut self) -> TextInput {
        let cell = std::rc::Rc::new(std::cell::RefCell::new((String::new(), false)));

        self.text_inputs.push(cell.clone());

        TextInput {
            text: cell,
        }
    }
}
