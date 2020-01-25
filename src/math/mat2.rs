macro_rules! impl_op {
    ($op_trait:ident, $op_func:ident, $op:tt) => {
        impl<I> $op_trait<I> for Mat2<I> 
            where I: $op_trait<I> + Copy,
                  <I as $op_trait<I>>::Output: Into<I>
        {
            type Output = Mat2<I>;

            fn $op_func(mut self, other: I) -> Self::Output {
                self.data[0][0] = (self.data[0][0] $op other).into();
                self.data[1][0] = (self.data[1][0] $op other).into();
                self.data[0][1] = (self.data[0][1] $op other).into();
                self.data[1][1] = (self.data[1][1] $op other).into();

                self
            }
        }

        impl<I> $op_trait<Mat2<I>> for Mat2<I> 
            where I: $op_trait<I> + Copy,
                  <I as $op_trait<I>>::Output: Into<I>
        {
            type Output = Mat2<I>;

            fn $op_func(mut self, other: Mat2<I>) -> Self::Output {
                self.data[0][0] = (self.data[0][0] $op other.data[0][0]).into();
                self.data[1][0] = (self.data[1][0] $op other.data[1][0]).into();
                self.data[0][1] = (self.data[0][1] $op other.data[0][1]).into();
                self.data[1][1] = (self.data[1][1] $op other.data[1][1]).into();

                self
            }
        }
    };
}

macro_rules! impl_op_ass {
    ($op_trait:ident, $op_func:ident, $op:tt) => {
        impl<I> $op_trait<I> for Mat2<I> 
            where I: $op_trait<I> + Copy
        {
            fn $op_func(&mut self, other: I) {
                self.data[0][0] $op other;
                self.data[1][0] $op other;
                self.data[0][1] $op other;
                self.data[1][1] $op other;
            }
        }

        impl<I> $op_trait<Mat2<I>> for Mat2<I> 
            where I: $op_trait<I> + Copy
        {

            fn $op_func(&mut self, other: Mat2<I>) {
                self.data[0][0] $op other.data[0][0];
                self.data[1][0] $op other.data[1][0];
                self.data[0][1] $op other.data[0][1];
                self.data[1][1] $op other.data[1][1];
            }
        }
    };
}

use std::f32::consts::PI;
use super::Vec2;
use std::ops::{
    Add,
    Sub,
    Mul,
    Div,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
};

pub struct Mat2<I> {
    data: [[I; 2]; 2],
}

impl<I> Mat2<I> {
    pub fn new(data: [[I; 2]; 2]) -> Mat2<I> {
        Mat2 {
            data
        }
    }

    pub fn from_radians(angle: f32) -> Mat2<f32> {
        Mat2 {
            data: [
                [angle.cos(), -angle.sin()],
                [angle.sin(), angle.cos()]
            ]
        }
    }

    pub fn from_degrees(mut angle: f32) -> Mat2<f32> {
        angle = angle/180.0 * PI;

        Self::from_radians(angle)
    }

    pub fn as_array(self) -> [[I; 2]; 2] {
        self.data
    }
}

impl<I> Mul<Vec2<I>> for Mat2<I> 
    where I: Add<I> + Mul<I> + Copy,
          <I as Mul<I>>::Output: Into<I>,
          <I as Add<I>>::Output: Into<I>
{
    type Output = Vec2<I>;

    fn mul(self, mut other: Vec2<I>) -> Self::Output {
        let x = other.x;
        other.x = ((other.x * self.data[0][0]).into() + (other.y * self.data[0][1]).into()).into();
        other.y = ((x * self.data[1][0]).into() + (other.y * self.data[1][1]).into()).into();

        other
    }
}

impl<I> Mul<Mat2<I>> for Vec2<I> 
    where I: Add<I> + Mul<I> + Copy,
          <I as Mul<I>>::Output: Into<I>,
          <I as Add<I>>::Output: Into<I>
{
    type Output = Vec2<I>;

    fn mul(mut self, other: Mat2<I>) -> Self::Output {
        let x = self.x;
        self.x = ((self.x * other.data[0][0]).into() + (self.y * other.data[0][1]).into()).into();
        self.y = ((x * other.data[1][0]).into() + (self.y * other.data[1][1]).into()).into();

        self
    }
}

impl<I> MulAssign<Mat2<I>> for Vec2<I>
    where Vec2<I>: Mul<Mat2<I>> + Clone,
          <Vec2<I> as Mul<Mat2<I>>>::Output: Into<Vec2<I>>
{
    fn mul_assign(&mut self, other: Mat2<I>) {
        *self = (self.clone() * other).into();
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
