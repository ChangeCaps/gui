pub struct Frame<'f> {
    frame: &'f mut glium::Frame,
}

impl<'f> Frame<'f> {
    pub fn new(frame: &'f mut glium::Frame) -> Self {
        Frame {
            frame,
        }
    }

    
}