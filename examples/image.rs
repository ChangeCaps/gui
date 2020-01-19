extern crate gui;

use gui::*;

struct ImageExample {
    image: Image,
    x: f32,
}

impl SimpleState for ImageExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.image(&self.image)
            //.scale(0.5)
            .position(math::Vec2::new(self.x, 0.0))
            //.anchor(Anchor::MiddleLeft)
            //.pivot(Anchor::MiddleLeft)
            .draw();

        self.x += 1.0 * _state_data.delta_time;
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(600, 400)
        .with_pixel_window_size(300, 200)
        .run(|loader| {
            Box::new(ImageExample {
                image: loader.load_image("assets/test_image.png", PNG),
                x: 0.0
            })
        });
}
