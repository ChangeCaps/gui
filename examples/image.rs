extern crate fumarole;

use fumarole::*;

#[derive(Clone)]
struct ImageExample {
    x: f32,
}

impl State for ImageExample {
    fn draw(&self, frame: &mut Frame, _state_data: &StateData) {
        frame.image("assets/test_image.png")
            .position(_state_data.mouse_position)
            .pixel_scale(1.0)
            .draw();
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_frame_size(100.0, 100.0)
        .run(|loader| {
            loader.load_image("assets/test_image.png", PNG);

            Box::new(ImageExample {
                x: 0.0
            })
        });
}
