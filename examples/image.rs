extern crate gui;

use gui::*;

struct ImageExample {
    x: f32,
}

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.image("assets/test_image.png")
            .scale(10.0)
            .position(math::Vec2::new(self.x, 0.0))
            //.anchor(Anchor::MiddleLeft)
            //.pivot(Anchor::MiddleLeft)
            .draw();

        self.x += 0.01 * _state_data.delta_time;
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(600, 400)
        .run(|loader| {
            loader.load_image("assets/test_image.png", PNG);

            Box::new(ImageExample {
                x: 0.0
            })
        });
}
