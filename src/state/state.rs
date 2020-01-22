use super::Transition;
use super::super::Frame;
use super::StateData;

pub trait State {
    fn draw(&mut self, _frame: &mut Frame, _state_data: &StateData) {
        
    }
    
    fn shadow_draw(&mut self, _frame: &mut Frame, _state_data: &StateData) {
        
    }

    fn update(&mut self, _state_data: &StateData) -> Transition {
        Transition::None
    }

    fn shadow_update(&mut self, _state_data: &StateData) {
        
    }
}
