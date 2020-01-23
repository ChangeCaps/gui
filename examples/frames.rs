extern crate gui;

use gui::*;
use math::*;

struct ImageExample {
    x: f32,
}

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, _state_data: &StateData) {
        let mut f = frame.new_frame();

        f.rect()
            .size(Vec2::new(2.0, 2.0))
            .color(color::rgb(1.0, 0.0, 0.0))
            .draw();

        f.rect()
            .draw();
        
        frame.draw_frame(f)
            .anchor(Anchor::TopRight)
            .draw();

        self.x += _state_data.delta_time * 0.01;
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(600, 400)
        .run(|loader| {
            loader.load_image("assets/test_image.png", PNG);

            Box::new(ImageExample {
                x: 0.0
            })
        });
}
