extern crate gui;

use gui::*;
use math::*;

struct ImageExample {
    font: Font,
}

impl State for ImageExample {
    fn draw(&mut self, mut frame: Frame, _data: StateData) -> Transition {
        frame.clear();

        frame.rect()
            .size(Vec2::new(0.7, 0.2))
            .color(color::rgb(0.7, 0.3, 0.2))
            .anchor(Anchor::MiddleRight)
            .pivot(Anchor::MiddleRight)
            .scaling(true)
            .draw();

        frame.text(&self.font)
            .position(Vec2::new(-0.1, 0.0))
            .scale(0.05)
            .anchor(Anchor::MiddleRight)
            .pivot(Anchor::MiddleRight)
            .scaling(true)
            .text("Hello World")
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