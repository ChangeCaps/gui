extern crate gui;

use gui::*;

struct ImageExample {
    image: Image,
}

impl State for ImageExample {
    fn draw(&mut self, mut frame: Frame, _state_data: StateData) -> Transition {
        frame.image(&self.image)
            .scale(0.5)
            .scaling(true)
            .draw();

        Transition::None
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