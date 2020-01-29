extern crate gui;

use gui::*;
use math::*;

struct PivotExample {
    rotation: f32
}

impl State for PivotExample { 
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        let mut t = Transform::new();

        t.position = Vec2::new(0.0, 0.0);
        t.rotation = self.rotation;

        let mut p = Transform::new();

        p.position = Vec2::new(0.5, 0.0);
        p.rotation = self.rotation;

        p = p.transform(t);

        frame.rect()
            .size(Vec2::new(0.2, 0.2))
            .parent(p)
            .draw();

        frame.rect()
            .position(Vec2::new(0.0, 0.5))
            .size(Vec2::new(0.3, 0.3))
            .parent(p)
            .draw();

        self.rotation += _state_data.delta_time;
    }
}

fn main() {
    Application::new()
        .with_title("Pivot Example")
        .with_window_size(1000, 800)
        .with_depth_sorting(true)
        .run(|_loader| {
            Box::new(PivotExample {
                rotation: 0.0
            })
        });
}
