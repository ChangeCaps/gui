extern crate gui;

use gui::*;

struct PivotExample;

impl State for PivotExample {
    fn draw(&mut self, mut frame: Frame, _state_data: StateData) -> Transition {
        frame.rect()
            .color(color::rgb(0.7, 0.3, 0.2))
            .draw();

        frame.rect()
            .color(color::rgb(0.2, 0.3, 0.7))
            .pivot(Anchor::BottomLeft)
            .draw();

        frame.rect()
            .color(color::rgb(0.3, 0.7, 0.2))
            .pivot(Anchor::TopRight)
            .rotation(45.0)
            .draw();

        Transition::None
    }
}

fn main() {
    Application::new()
        .with_title("Pivot Example")
        .with_window_size(1000, 800)
        .run(|loader| {
            Box::new(PivotExample)
        });
}