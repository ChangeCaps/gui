extern crate gui;

use gui::*;
use math::*;

struct Square;

impl State for Square {
    fn draw(&mut self, mut frame: Frame, state_data: StateData) -> Transition {
        frame.clear();

        frame.line()
            .points(
                Vec2::new(0.2, 0.2),
                Vec2::new(-0.2, -0.2),
            )
            .scaling()
            .width(0.05)
            .smooth()
            .anchor(Anchor::MiddleLeft)
            .draw();

        frame.ellipse()
            .scaling()
            .position(state_data.scaled_mouse_position)
            .size(Vec2::new(0.1, 0.1))
            .draw();

        Transition::None
    }
}

fn main() {
    let square = Square;

    Application::new()
        .with_title("Image test")
        .not_resizable()
        .with_window_size(1000, 400)
        .run(Box::new(square));
}