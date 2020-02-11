//! The State is the basis for pretty much anything in fumarole. The application keeps track of
//! your states and runs functions such as draw and update on them.

mod state;
mod transition;
mod state_data;

pub use state::*;
pub use transition::*;
pub use state_data::*;
