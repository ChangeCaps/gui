use super::State;

pub enum Transition {
    None,
    Pop,
    Push(Box<dyn State + Send>),
    Trans(Box<dyn State + Send>),
}