extern crate gui;

use gui::*;

struct Push;

impl State for Push {
    fn draw(&mut self, _frame: Frame, _state_data: StateData) -> Transition {
        println!("Pushing State");

        Transition::Push(Box::new(Trans))
    }
}

struct Trans;

impl State for Trans {
    fn draw(&mut self, _frame: Frame, _state_data: StateData) -> Transition {
        println!("Transitioning State");
        Transition::Trans(Box::new(Pop))
    }
}

struct Pop;

impl State for Pop {
    fn draw(&mut self, _frame: Frame, _state_data: StateData) -> Transition {
        println!("Popping State");
        Transition::Pop
    }
}

#[test]
fn states() {
    Application::new()
        .run(|_| {
            Box::new(Push)
        });
}
