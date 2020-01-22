extern crate gui;

use gui::*;
use math::*;

struct PivotExample {

}

impl State for PivotExample { 
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.rect()
            .color(color::rgba(0.7, 0.3, 0.2, 0.3))
            .depth(0.5)
            .draw();

        frame.rect()
            .color(color::rgba(0.2, 0.3, 0.7, 0.3))
            .pivot(Anchor::BottomLeft)
            .draw();

        frame.ellipse()
            .size(Vec2::new(0.5, 0.5))
            .draw();

        for _ in 0..1 {
            frame.rect()
                .color(color::rgba(0.3, 0.7, 0.2, 1.0))
                .pivot(Anchor::TopRight)
                .rotation(45.0)
                .draw();
        }
    }
}

fn main() {
    Application::new()
        .with_title("Pivot Example")
        .with_window_size(1000, 800)
        .with_depth_sorting(true)
        .run(|_loader| {
            Box::new(PivotExample {

            })
        });
}
