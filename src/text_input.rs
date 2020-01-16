use std::cell::RefCell;
use std::rc::Rc;

pub struct TextInput {
    pub(crate) text: Rc<RefCell<(String, bool)>>,
}

impl TextInput {
    pub fn get_text(&self) -> String {
        self.text.borrow().0.clone()
    }

    pub fn start(&mut self) {
        *self.text.borrow_mut() = (String::new(), true);
    }

    pub fn stop(&mut self) {
        self.text.borrow_mut().1 = false;
    }
}
