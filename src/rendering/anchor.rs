use super::super::math::*;

#[derive(Clone, Copy)]
pub enum Anchor {
    Middle,
    TopLeft,
    TopMiddle,
    TopRight,
    MiddleRight,
    BottomRight,
    BottomMiddle,
    BottomLeft,
    MiddleLeft,
    Custom(Vec2<f32>),
}

impl Anchor {
    pub(crate) fn as_vec(self) -> Vec2<f32> {
        use Anchor::*;

        match self {
            Middle => Vec2::new(0.0, 0.0),
            TopLeft => Vec2::new(-1.0, 1.0),
            TopMiddle => Vec2::new(0.0, 1.0),
            TopRight => Vec2::new(1.0, 1.0),
            MiddleRight => Vec2::new(1.0, 0.0),
            BottomRight => Vec2::new(1.0, -1.0),
            BottomMiddle => Vec2::new(0.0, -1.0),
            BottomLeft => Vec2::new(-1.0, -1.0),
            MiddleLeft => Vec2::new(-1.0, 0.0),
            Custom(vec) => vec
        }
    }
}