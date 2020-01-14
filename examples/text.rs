extern crate gui;

use gui::*;

struct ImageExample {
    font: Font,
}

impl State for ImageExample {
    fn draw(&mut self, mut frame: Frame, _state_data: StateData) -> Transition {
        frame.rect()
            .color(color::rgb(0.0, 1.0, 0.0))
            .draw();

        frame.text(&self.font)
            .text("Ihe")
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
                font: loader.load_font("assets/test_font.ttf", 1000)
            })
        });
}