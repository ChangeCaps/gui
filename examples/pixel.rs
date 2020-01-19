extern crate gui;

use gui::*;
use math::*;

struct PixelExample;

impl SimpleState for PixelExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        frame.ellipse()
            .draw();

        frame.rect()
            .rotation(45.0)
            .draw();

        frame.line()
            .smooth(true)
            .width(0.1)
            .points(
                Vec2::new(0.5, 0.0),
                Vec2::new(0.0, 0.5)
            )
            .draw();
    }
}

fn main() {
    Application::new()
        .with_title("Pixel Example")
        .with_window_size(600, 400)
        //.with_pixel_window_size(300, 200)
        .run(|_loader| {
            Box::new(PixelExample)
        });
}
