use super::Transition;
use super::super::Frame;
use super::StateData;

pub trait State {
    fn draw(&mut self, frame: Frame, state_data: StateData) -> Transition;
}