extern crate gui;

use gui::*;

#[derive(Clone)]
struct ImageExample {
    x: f32,
}

impl State for ImageExample {
    fn draw(&self, frame: &mut Frame, _state_data: &StateData) {
        frame.image("assets/test_image.png")
            .position(_state_data.mouse_position)
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
