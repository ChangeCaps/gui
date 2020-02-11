//! fumarole is a simple engine for simple games and gui applications alike.
//! It uses glium for rendering, and it is quite efficient size it's a 2d engine
//! and draws everything in the same draw call.
//!
//! # Example
//! ```
//! extern crate fumarole;
//!
//! use fumarole::*;
//!
//! struct Test {
//!     x: f32
//! };
//!
//! impl State for Test {
//!     fn draw(&self, frame: &mut Frame, _data: &StateData) {
//!         frame.rect()
//!             .position(Vec2::new(self.x, 0.0))
//!             .draw();
//!     }
//! }
//!
//! fn main() {
//!     Application::new()
//!         .run(|_loader| {
//!             Box::new(Test {
//!                 x: -1.0
//!             }) 
//!         });
//! }
//! ```

#[macro_use]
extern crate glium;
extern crate image;
extern crate rusttype;
extern crate serde;

pub mod application;
pub mod state;
pub mod color;
pub mod math;
pub mod rendering;
mod loader;
mod font;
mod text_input;
pub(crate) mod drawing_data;
pub(crate) mod texture_atlas;
mod transform;

pub type KeyCode = glium::glutin::event::VirtualKeyCode;

pub(crate) use rendering::RECT_VERTS;

pub use application::*;
pub use state::*;
pub use rendering::{
    Anchor,
    Frame,
    Canvas,
};
pub use image::ImageFormat::*;
pub use loader::*;
pub use font::*;
pub use glium::glutin::event::MouseButton;
pub use text_input::*;
pub(crate) use drawing_data::*;
pub use transform::*;
pub use math::*;
