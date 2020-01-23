extern crate gui;

use gui::*;
use math::*;

struct ImageExample;

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, data: &StateData) {
        for _ in 0..1 {
            frame.rect()
                .draw();
        }

        println!("{}", 1.0 / data.delta_time);
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(1000, 600)
        .with_depth_sorting(true)
        .run(|loader| {
            loader.load_image("assets/test_image.png", PNG);

            Box::new(ImageExample)
        });
}
