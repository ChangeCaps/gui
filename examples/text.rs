extern crate gui;

use gui::*;
use math::*;

struct TextExample {
    text_input: TextInput,
}

impl State for TextExample {
    fn draw(&mut self, frame: &mut Frame, data: &StateData) {
        frame.rect()
            .size(Vec2::new(0.7, 0.2))
            .color(color::rgb(0.7, 0.3, 0.2))
            .anchor(Anchor::MiddleRight)
            .pivot(Anchor::MiddleRight)
            .scaling(true)
            .draw();

        frame.text("assets/test_font.ttf")
            .position(Vec2::new(-0.1, 0.0))
            .scale(0.05)
            .anchor(Anchor::MiddleRight)
            .pivot(Anchor::MiddleRight)
            .scaling(true)
            .color(
                if data.key_held(KeyCode::A) {
                    color::rgb(1.0, 1.0, 1.0)
                } else {
                    color::rgb(0.0, 0.0, 1.0)
                }
            )
            .text(self.text_input.get_text())
            .draw();
    }
}

fn main() {
    Application::new()
        .with_title("Text Example")
        .with_window_size(1000, 800)
        .run(|loader| {
            let mut text_input = loader.text_input();
            text_input.start();

            loader.load_font("assets/test_font.ttf", 1000);

            Box::new(TextExample {
                text_input,
            })
        });
}
