extern crate gui;

use gui::*;
use math::*;

struct ImageExample;

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, data: &StateData) {   
        for i in 0..10_000 {
            frame.image("assets/ship.png")
                .draw();
        }
        
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(1000, 600)
        .with_depth_sorting(true)
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
