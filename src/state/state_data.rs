use super::super::*;
use math::*;

pub struct StateData {
    pub delta_time: f32, 
    pub frame_dimensions: Vec2<f32>,
    pub window_dimensions: Vec2<f32>,
    pub aspect_ratio: f32,
    pub mouse_position: Vec2<f32>,
    pub scaled_mouse_position: Vec2<f32>,
}