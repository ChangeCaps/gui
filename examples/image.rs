extern crate gui;

use gui::*;

struct Square;

impl State for Square {
    fn draw(&mut self, mut frame: Frame, state_data: StateData) -> Transition {
        frame.rect()
            .color(color::rgb(0.2, 0.7, 0.5))
            .size(state_data.window_dimensions - (0.1, 0.1))
            .draw();

        Transition::None
    }
}

fn main() {
    let square = Square;

    Application::new()
        .run(Box::new(square));
}