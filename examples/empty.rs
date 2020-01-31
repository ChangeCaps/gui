extern crate gui;

use gui::*;
use math::*;

struct ImageExample {
    x: f32
}

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, data: &StateData) {   
        for i in 0..100_000 {
            frame.rect()
                .position(Vec2::new(i as f32 / 10_000.0 - 0.5 + self.x, i as f32 / 10_000.0 - 0.5))
                .draw();
        }

        self.x += data.delta_time;
        println!("{}", 1.0/data.delta_time);
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(1000, 600)
        .with_depth_sorting(false)
        .run(|loader| {
            loader.load_image("assets/ship.png", PNG);
            loader.load_image("assets/test_image.png", PNG);
            
            loader.load_image("assets/brown_planet.png", PNG);
            loader.load_image("assets/door.png", PNG);
            loader.load_image("assets/minimap_frame.png", PNG);
            loader.load_image("assets/red_planet.png", PNG);
            loader.load_image("assets/satalite.png", PNG);
            loader.load_image("assets/ship_part.png", PNG);
            loader.load_image("assets/spider_planet.png", PNG);
            loader.load_image("assets/velocity_arrow.png", PNG);

            Box::new(ImageExample {
                x: 0.0,
            })
        });
}
