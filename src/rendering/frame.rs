use crate::{
    rendering::*,
};

pub trait Canvas<'f> {
    fn drawing_data(&mut self) -> &mut DrawingData;

    fn rect(&mut self) -> Rect {
        let masks = self.masks();

        Rect::new(self.drawing_data(), masks)
    }

    fn line(&mut self) -> Line {
        let masks = self.masks();

        Line::new(self.drawing_data(), masks)
    }

    fn ellipse(&mut self) -> Ellipse {
        let masks = self.masks();

        Ellipse::new(self.drawing_data(), masks)
    }

    fn image<P>(&mut self, image: P) -> Image
        where P: Into<String>
    {
        let masks = self.masks();

        Image::new(self.drawing_data(), image.into(), masks)
    }

    fn text<P>(&mut self, font: P) -> Text
        where P: Into<String>
    {
        let masks = self.masks();

        Text::new(self.drawing_data(), font.into(), masks)
    }

    fn masks(&self) -> (i32, i32);

    fn rect_mask(&mut self) -> RectMask {
        RectMask::new(self.drawing_data())
    }
}

pub struct Frame<'f> { 
    pub(crate) drawing_data: &'f mut DrawingData,
}

impl<'f> Canvas<'f> for Frame<'f> {
    fn drawing_data(&mut self) -> &mut DrawingData {
        self.drawing_data
    }
     
    fn masks(&self) -> (i32, i32) {
        (0, 0)
    }
}
