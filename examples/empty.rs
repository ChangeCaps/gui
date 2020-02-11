extern crate gui;

use gui::*;
use math::*;

#[derive(Clone)]
struct ImageExample {
    x: f32
}

impl State for ImageExample {
    fn draw(&self, frame: &mut Frame, data: &StateData) {   
        for i in 0..100_000 {
            frame.ellipse()
                .position(Vec2::new(i as f32 / 10_000.0 - 0.5 + self.x, i as f32 / 10_000.0 - 0.5))
                .draw();
        }

        frame.text("assets/test_font.ttf")
            .text(format!("fps: {}", 1.0/data.delta_time))
            .color(color::rgb(1.0, 0.0, 0.0))
            .size(Vec2::new(0.1, 0.1))
            .anchor(Anchor::TopLeft)
            .pivot(Anchor::TopLeft)
            .draw();
    }

    fn update(&mut self, data: &StateData) -> Transition { 
        self.x += 0.1 * data.delta_time;

        Transition::None
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(1000, 600)
        .with_depth_sorting(true)
        .run(|loader| {
            loader.load_font("assets/test_font.ttf", 100);

            Box::new(ImageExample {
                x: 0.0,
            })
        });
}
