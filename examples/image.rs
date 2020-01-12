extern crate gui;

use gui::*;
use math::*;

struct Square;

impl State for Square {
    fn draw(&mut self, mut frame: Frame, state_data: StateData) -> Transition {
        frame.line()
            .points(
                Vec2::new(0.7, 0.8),
                Vec2::new(0.0, 0.2)
            )
            .anchor(Anchor::BottomLeft)
            .draw();

        Transition::None
    }
}

fn main() {
    let square = Square;

    Application::new()
        .run(Box::new(square));
}