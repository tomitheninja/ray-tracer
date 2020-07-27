// If you want to implement T [operator] Vec3<T> append T to the macro calls as last argument
use num::Float;
use std::ops::{Index, IndexMut};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T>(T, T, T);

impl<T> Vec3<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self(a, b, c)
    }
}

impl<T: Copy> From<T> for Vec3<T> {
    fn from(x: T) -> Self {
        Vec3::new(x, x, x)
    }
}

impl<T: Float> Vec3<T> {
    pub fn length(&self) -> T {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn dot(&self, other: &Self) -> T {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }
}

impl<T> Index<usize> for Vec3<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Vec3 has no such index"),
        }
    }
}

impl<T> IndexMut<usize> for Vec3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Vec3 has no such index"),
        }
    }
}

macro_rules! impl_op {
    ($target:tt, $op:tt, $fn_name:tt) => {
        // Implement Self<T> + Self<T>
        impl<T: Copy + std::ops::$op<Output = T>> std::ops::$op<Self> for $target<T> {
            type Output = Self;

            fn $fn_name(self, rhs: Self) -> Self::Output {
                Self(
                    std::ops::$op::$fn_name(self.0, rhs.0),
                    std::ops::$op::$fn_name(self.1, rhs.1),
                    std::ops::$op::$fn_name(self.2, rhs.2),
                )
            }
        }

        // Implement Self<T> + T
        impl<T: Copy + std::ops::$op<Output = T>> std::ops::$op<T> for $target<T> {
            type Output = Self;

            fn $fn_name(self, rhs: T) -> Self::Output {
                Self(
                    std::ops::$op::$fn_name(self.0, rhs),
                    std::ops::$op::$fn_name(self.1, rhs),
                    std::ops::$op::$fn_name(self.2, rhs),
                )
            }
        }
    };

    ($target:tt, $op:tt, $fn_name:tt, $assign_op:tt, $assign_fn_name:tt) => {
        impl_op!($target, $op, $fn_name);

        // Implement Self<T> += Self<T>
        impl<T: Copy + std::ops::$op<Output = T>> std::ops::$assign_op for $target<T> {
            fn $assign_fn_name(&mut self, rhs: Self) {
                *self = std::ops::$op::$fn_name(*self, rhs);
            }
        }

        // Implement Self<T> += T
        impl<T: Copy + std::ops::$op<Output = T>> std::ops::$assign_op<T> for $target<T> {
            fn $assign_fn_name(&mut self, rhs: T) {
                *self = std::ops::$op::$fn_name(*self, rhs);
            }
        }
    };
}

impl_op!(Vec3, Add, add, AddAssign, add_assign);
impl_op!(Vec3, Sub, sub, SubAssign, sub_assign);
impl_op!(Vec3, Mul, mul, MulAssign, mul_assign);
impl_op!(Vec3, Div, div, DivAssign, div_assign);
