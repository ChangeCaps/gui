extern crate gui;

use gui::*;
use math::*;

struct ShapeExample;

impl State for ShapeExample {
    fn draw(&mut self, mut frame: Frame, state_data: StateData) -> Transition {
        frame.clear();

        frame.line()
            .points(
                Vec2::new(0.2, 0.2),
                Vec2::new(-0.2, -0.2),
            )
            .scaling(true)
            .width(0.05)
            .smooth(true)
            .draw();

        frame.ellipse()
            .scaling(true)
            .position(state_data.scaled_mouse_position)
            .size(Vec2::new(0.1, 0.1))
            .draw();

        Transition::None
    }
}

fn main() {
    Application::new()
        .with_title("Shapes Example")
        .not_resizable()
        .with_window_size(1000, 800)
        .run(|_| {

            Box::new(ShapeExample)
        });
}