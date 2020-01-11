use super::Transition;
use super::super::Frame;

pub trait State {
    fn draw(&mut self, frame: Frame) -> Transition;
}