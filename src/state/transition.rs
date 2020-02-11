//! Transitions between states are handled using the aptly named enum

use super::State;

pub enum Transition {
    None,
    Pop,
    Push(Box<dyn State + Send>),
    Trans(Box<dyn State + Send>),
}
