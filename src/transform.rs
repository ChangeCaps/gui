use crate::*;
use math::*;

#[derive(Clone, Copy, Debug, Default)]
pub struct Transform {
    pub position: Vec2<f32>,
    pub rotation: f32,
    pub size: Vec2<f32>,
}

impl Transform {
    pub fn new() -> Self {
        Transform {
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            size: Vec2::new(1.0, 1.0),
        }
    }

    pub fn transform(mut self, other: Transform) -> Self {
        self.position *= other.size;
        self.position *= Mat2::<f32>::from_radians(other.rotation);
        self.position += other.position;
        self.rotation += other.rotation;

        self
    }
}
