use crate::{
    rendering::*,
};

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

    fn image<P>(&mut self, image: P) -> Image
        where P: Into<String>
    {
        Image::new(self.drawing_data(), image.into())
    }

    fn text<P>(&mut self, font: P) -> Text
        where P: Into<String>
    {
        Text::new(self.drawing_data(), font.into())
    }

    fn masks(&self) -> (i32, i32);
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
