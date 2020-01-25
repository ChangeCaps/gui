extern crate gui;

use gui::*;
use math::*;

struct ImageExample;

impl State for ImageExample {
    fn draw(&mut self, frame: &mut Frame, data: &StateData) {   
        for i in 0..10_000 {
            frame.line()
                .draw();
        }
        
        println!("{}", 1.0/data.delta_time);
    }
}

fn main() {
    Application::new()
        .with_title("Image Example")
        .with_window_size(1000, 600)
        .with_depth_sorting(true)
        .run(|loader| {
            

            Box::new(ImageExample)
        });
}
