#[macro_use]
extern crate glium;
extern crate image;
extern crate rusttype;

pub mod application;
pub mod state;
pub mod frame;
pub mod color;
pub mod math;
pub mod rendering;
mod loader;
mod font;
mod text_input;
pub(crate) mod drawing_data;

pub type KeyCode = glium::glutin::event::VirtualKeyCode;

pub use application::*;
pub use state::*;
pub use frame::*;
pub use rendering::{
    Anchor,
    Shape,
};
pub use image::ImageFormat::*;
pub use loader::*;
pub use font::*;
pub use glium::glutin::event::MouseButton;
pub use text_input::*;
pub(crate) use drawing_data::*;
