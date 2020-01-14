use super::super::*;
use math::*;
use std::collections::HashSet;

pub struct StateData<'s> {
    pub delta_time: f32, 
    pub frame_dimensions: Vec2<f32>,
    pub scaled_frame_dimensions: Vec2<f32>,
    pub window_dimensions: Vec2<f32>,
    pub aspect_ratio: f32,
    pub mouse_position: Vec2<f32>,
    pub scaled_mouse_position: Vec2<f32>,
    pub keys_pressed: &'s HashSet<KeyCode>,
    pub keys_held: &'s HashSet<KeyCode>,
    pub keys_released: &'s HashSet<KeyCode>,
    pub mouse_buttons_pressed: &'s HashSet<MouseButton>,
    pub mouse_buttons_held: &'s HashSet<MouseButton>,
    pub mouse_buttons_released: &'s HashSet<MouseButton>,
}

impl<'s> StateData<'s> {
    pub fn key_pressed(&self, key_code: KeyCode) -> bool {
        self.keys_pressed.contains(&key_code)
    }

    pub fn key_held(&self, key_code: KeyCode) -> bool {
        self.keys_held.contains(&key_code)
    }
    
    pub fn key_released(&self, key_code: KeyCode) -> bool {
        self.keys_released.contains(&key_code)
    }

    pub fn mouse_pressed(&self, mouse_button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&mouse_button)
    }

    pub fn mouse_held(&self, mouse_button: MouseButton) -> bool {
        self.mouse_buttons_held.contains(&mouse_button)
    }

    pub fn mouse_released(&self, mouse_button: MouseButton) -> bool {
        self.mouse_buttons_released.contains(&mouse_button)
    }
}
