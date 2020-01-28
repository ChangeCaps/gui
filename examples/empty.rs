extern crate gui;

use gui::*;
use math::*;

struct ImageExample;

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, data: &StateData) {   
        for i in 0..50_000 {
            frame.ellipse()
                .position(Vec2::new(i as f32 / 10_000.0 - 0.5, i as f32 / 10_000.0 - 0.5))
                .draw();
        }
        
        println!("\r{}", 1.0/data.delta_time);
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

            Box::new(ImageExample)
        });
}
