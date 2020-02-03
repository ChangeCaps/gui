extern crate gui;

use gui::*;
use math::*;

#[derive(Clone)]
struct ImageExample {
    x: f32,
}

impl State for ImageExample {
    fn draw(&self, frame: &mut Frame, _state_data: &StateData) {
        let mut mask = frame.rect_mask()
            .draw();

        mask.ellipse()
            .position(Vec2::new(self.x, 0.0))
            .draw();
    }

    fn update(&mut self, data: &StateData) -> Transition {
        self.x += data.delta_time * 0.3;

        Transition::None
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(600, 400)
        .run(|loader| {
            loader.load_image("assets/test_image.png", PNG);

            Box::new(ImageExample {
                x: -2.0
            })
        });
}
