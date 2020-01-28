macro_rules! impl_op {
    ($op_trait:ident, $op_func:ident, $op:tt) => {
        impl<I> $op_trait<I> for Vec2<I> 
            where I: $op_trait<I> + Copy,
                  <I as $op_trait<I>>::Output: Into<I>
        {
            type Output = Vec2<I>;
        
            fn $op_func(mut self, other: I) -> Self::Output {
                self.x = (self.x $op other).into();
                self.y = (self.y $op other).into();
            
                self
            }
        }

        impl<I> $op_trait<(I, I)> for Vec2<I> 
            where I: $op_trait<I> + Copy,
                  <I as $op_trait<I>>::Output: Into<I>
        {
            type Output = Vec2<I>;
        
            fn $op_func(mut self, other: (I, I)) -> Self::Output {
                self.x = (self.x $op other.0).into();
                self.y = (self.y $op other.1).into();
            
                self
            }
        }

        impl<I> $op_trait<Vec2<I>> for Vec2<I>
            where I: $op_trait<I> + Copy,
                  <I as $op_trait<I>>::Output: Into<I>
        {
            type Output = Vec2<I>;
        
            fn $op_func(mut self, other: Vec2<I>) -> Self::Output {
                self.x = (self.x $op other.x).into();
                self.y = (self.y $op other.y).into();
            
                self
            }
        }
    };
}

macro_rules! impl_op_ass {
    ($op_trait:ident, $op_func:ident, $op:tt) => {
        impl<I> $op_trait<I> for Vec2<I> 
            where I: $op_trait<I> + Copy,
        {        
            fn $op_func(&mut self, other: I) {
                self.x $op other;
                self.y $op other;
            }
        }

        impl<I> $op_trait<(I, I)> for Vec2<I> 
            where I: $op_trait<I> + Copy,
        {        
            fn $op_func(&mut self, other: (I, I)) {
                self.x $op other.0;
                self.y $op other.1;
            }
        }

        impl<I> $op_trait<Vec2<I>> for Vec2<I> 
            where I: $op_trait<I> + Copy,
        {        
            fn $op_func(&mut self, other: Vec2<I>) {
                self.x $op other.x;
                self.y $op other.y;
            }
        }
    };
}

macro_rules! impl_float {
    ($float:ident) => {
        impl Vec2<$float> {
            #[inline]
            pub fn magnitude(self) -> $float {
                (self.x * self.x + self.y * self.y).sqrt()
            }

            #[inline]
            pub fn magnitude_squared(self) -> $float {
                self.x * self.x + self.y * self.y
            }

            #[inline]
            pub fn normalize(self) -> Vec2<$float> {
                self / self.magnitude()
            }

            #[inline]
            pub fn from_radians(angle: $float) -> Vec2<$float> {
                Vec2::new(angle.cos(), angle.sin())
            }

            #[inline]
            pub fn dot(self, other: Self) -> $float {
                self.x * other.x + self.y * other.y
            }

            #[inline]
            pub fn project(self, other: Self) -> Self {
                other * self.dot(other) / other.magnitude_squared()
            }
        }
    };
}

use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    Neg
};
pub use crate::math::*;
pub use crate::Transform;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Vec2<I> {
    pub x: I,
    pub y: I,
}

impl<I> Vec2<I> {
    pub fn new(x: I, y: I) -> Vec2<I> {
        Vec2 {
            x,
            y,
        }
    }

    pub fn as_array(self) -> [I; 2] {
        [self.x, self.y]
    }
}

impl Vec2<f32> {
    pub fn transform(mut self, transform: Transform) -> Self {
        self *= transform.size;
        self *= Mat2::<f32>::from_radians(transform.rotation);
        self += transform.position;

        self
    }
}

impl<I> Into<[I; 2]> for Vec2<I> {
    fn into(self) -> [I; 2] {
        self.as_array()
    }
}

impl<I> Neg for Vec2<I> 
    where I: Neg
{
    type Output = Vec2<<I as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Vec2::new(-self.x, -self.y)
    }
}

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op!(Mul, mul, *);
impl_op!(Div, div, /);

impl_op_ass!(AddAssign, add_assign, +=);
impl_op_ass!(SubAssign, sub_assign, -=);
impl_op_ass!(MulAssign, mul_assign, *=);
impl_op_ass!(DivAssign, div_assign, /=);

impl_float!(f32);
impl_float!(f64);
