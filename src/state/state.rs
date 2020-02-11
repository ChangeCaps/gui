use super::Transition;
use super::super::Frame;
use super::StateData;

pub trait State: Send + StateClone {
    fn draw(&self, _frame: &mut Frame, _state_data: &StateData) {
        
    }
    
    fn shadow_draw(&self, _frame: &mut Frame, _state_data: &StateData) {
        
    }

    fn update(&mut self, _state_data: &StateData) -> Transition {
        Transition::None
    }

    fn shadow_update(&mut self, _state_data: &StateData) {
        
    }
}

pub trait StateClone {
    fn clone_box(&self) -> Box<dyn State>;
}

impl<T> StateClone for T 
    where T: State + Clone + 'static
{
    fn clone_box(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn State> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
