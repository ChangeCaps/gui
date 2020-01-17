extern crate gui;

use gui::*;

struct ImageExample {
    image: Image,
}

impl SimpleState for ImageExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.image(&self.image)
            .scale(0.5)
            .scaling(true)
            .anchor(Anchor::MiddleLeft)
            .pivot(Anchor::MiddleLeft)
            .draw();
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(1000, 800)
        .run(|loader| {
            Box::new(ImageExample {
                image: loader.load_image("assets/test_image.png", PNG)
            })
        });
}
