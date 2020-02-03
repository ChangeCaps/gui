mod rect_mask;

pub use rect_mask::*;

use crate::*;

pub struct Mask<'m> {
    masks: (i32, i32),
    drawing_data: &'m mut DrawingData,
}

impl<'m> Canvas<'m> for Mask<'m> {
    fn drawing_data(&mut self) -> &mut DrawingData {
        self.drawing_data
    }

    fn masks(&self) -> (i32, i32) {
        self.masks
    }
}
