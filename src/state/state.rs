use super::Transition;
use super::super::Frame;
use super::StateData;

pub trait State {
    fn draw(&mut self, frame: &mut Frame, state_data: &StateData);

    fn update(&mut self, state_data: &StateData) -> Transition;

    fn shadow_update(&mut self, state_data: &StateData);
}

pub trait SimpleState {
    fn draw(&mut self, frame: &mut Frame, state_data: &StateData);
}

impl<T> State for T 
    where T: SimpleState
{
    fn draw(&mut self, frame: &mut Frame, state_data: &StateData) {
        self.draw(frame, state_data);
    }
 
    fn update(&mut self, _state_data: &StateData) -> Transition {
        Transition::None
    }

    fn shadow_update(&mut self, _state_data: &StateData) {
        
    }
}
