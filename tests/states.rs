extern crate gui;

use gui::*;

struct Push;

impl State for Push {
    fn update(&mut self, _: &StateData) -> Transition {
        println!("Pushing State");

        Transition::Push(Box::new(Trans))
    }
}

struct Trans;

impl State for Trans {
    fn update(&mut self, _: &StateData) -> Transition {
        println!("Transitioning State");
        Transition::Trans(Box::new(Pop))
    }
}

struct Pop;

impl State for Pop {
    fn update(&mut self, _: &StateData) -> Transition {
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
