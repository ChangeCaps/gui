extern crate gui;

use gui::*;

struct ImageExample {
    x: f32,
}

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.image("assets/test_image.png")
            //.position(math::Vec2::new(self.x, 0.0))
            //.anchor(Anchor::MiddleLeft)
            //.pivot(Anchor::MiddleLeft)
            .draw();
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(600, 400)
        .with_fps(30.0)
        .run(|loader| {
            loader.load_image("assets/test_image.png", PNG);

            Box::new(ImageExample {
                x: 0.0
            })
        });
}
