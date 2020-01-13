extern crate gui;

use gui::*;
use math::*;

struct ShapeExample;

impl State for ShapeExample {
    fn draw(&mut self, mut frame: Frame, state_data: StateData) -> Transition {
        frame.clear();

        {
            let p0 = Vec2::new(state_data.scaled_frame_dimensions.x - 0.2, -state_data.mouse_position.y);
            let p1 = state_data.scaled_mouse_position;

            frame.line()
                .points(p0, p1)
                .scaling(true)
                .width(0.05)
                .smooth(true)
                .draw();

            frame.ellipse()
                .scaling(true)
                .position((p1 + p0) / 2.0)
                .size(Vec2::new(0.1, 0.2))
                .color(color::rgba(0.7, 0.0, 0.2, 0.6))
                .draw();

            frame.line()
                .points(
                    Vec2::new(-0.2, 0.8),
                    Vec2::new(-0.2, -0.8),
                )
                .scaling(true)
                .width(0.05)
                .smooth(true)
                .anchor(Anchor::MiddleRight)
                .draw();
        }
        
        frame.line()
            .points(
                Vec2::new(0.2, 0.8),
                Vec2::new(0.2, -0.8),
            )
            .scaling(true)
            .width(0.05)
            .smooth(true)
            .anchor(Anchor::MiddleLeft)
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
