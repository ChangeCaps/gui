use super::State;

pub enum Transition {
    None,
    Pop,
    Push(Box<dyn State>),
    Trans(Box<dyn State>),
}