extern crate gui;

use gui::*;
use math::*;

struct ShapeExample;

impl SimpleState for ShapeExample {
    fn draw(&mut self, frame: &mut Frame, state_data: &StateData) {
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

            frame.rect()
                .scaling(true)
                .position((p1 + p0) / 2.0)
                .size(Vec2::new(0.1, 0.5))
                .color(color::rgba(0.7, 0.0, 0.2, 0.6))
                .depth(0.5)
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
