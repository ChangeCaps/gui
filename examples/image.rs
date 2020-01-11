extern crate gui;

use gui::*;

struct Square;

impl State for Square {
    fn draw(&mut self, _frame: Frame) -> Transition {


        Transition::None
    }
}

fn main() {
    let app = gui::Application::new().build();

    let square = Square;

    app.run(Box::new(square));
}