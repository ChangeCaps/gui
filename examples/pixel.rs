extern crate gui;

use gui::*;
use math::*;

struct PixelExample;

impl State for PixelExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.ellipse()
            .size(Vec2::new(50.0, 50.0))
            .draw();

        frame.rect()
            .size(Vec2::new(50.0, 50.0))
            .color(color::rgb(0.0, 1.0, 0.0))
            .draw();
    }
}

fn main() {
    Application::new()
        .with_title("Pixel Example")
        .with_window_size(600, 400)
        .with_pixel_window_size(600, 400)
        .run(|_loader| {
            Box::new(PixelExample)
        });
}
