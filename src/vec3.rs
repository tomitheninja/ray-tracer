#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vec3(f64, f64, f64);

pub use Vec3 as Color;
pub use Vec3 as Point;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn random() -> Self {
        use ::rand::random;
        Self(random(), random(), random())
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        use ::rand::random;
        let r = || min + (max - min) * random::<f64>();
        Self(r(), r(), r())
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random_range(-1.0, 1.0);
            if p.len_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vec() -> Self {
        use ::rand::random;
        let r = |min, max| min + (max - min) * random::<f64>();
        let a: f64 = r(0.0, 2.0 * std::f64::consts::PI);
        let z = r(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self(r * a.cos(), r * a.sin(), z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn len_squared(&self) -> f64 {
        self.x().powi(2) + self.y().powi(2) + self.z().powi(2)
    }

    pub fn dot(u: &Self, v: &Self) -> f64 {
        (0..3).map(|i| u[i] * v[i]).sum()
    }

    pub fn cross(u: &Self, v: &Self) -> Self {
        Self::new(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0,
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    fn to_u8(x: f64) -> u8 {
        (256.0 * x.sqrt().min(0.999).max(0.0)) as u8
    }

    pub fn rgb_bytes(&self, samples_per_pixel: usize) -> (u8, u8, u8) {
        let scale = 1.0 / (samples_per_pixel as f64);
        (
            Self::to_u8(self.0 * scale),
            Self::to_u8(self.1 * scale),
            Self::to_u8(self.2 * scale),
        )
    }

    pub fn rgb(&self, samples_per_pixel: usize) -> String {
        let (r, g, b) = self.rgb_bytes(samples_per_pixel);
        format!("{} {} {}", r, g, b)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

use std::ops::{Add, Div, Mul, Sub};
macro_rules! impl_immutable_op {
    ($target:ident, $trait_name:ident, $trait_fn:ident) => {
        impl $trait_name<Self> for $target {
            type Output = Self;

            fn $trait_fn(self, rhs: Self) -> Self::Output {
                Self::new(
                    $trait_name::$trait_fn(self.0, rhs.0),
                    $trait_name::$trait_fn(self.1, rhs.1),
                    $trait_name::$trait_fn(self.2, rhs.2),
                )
            }
        }

        impl $trait_name<f64> for $target {
            type Output = Self;

            fn $trait_fn(self, rhs: f64) -> Self::Output {
                Self::new(
                    $trait_name::$trait_fn(self.0, rhs),
                    $trait_name::$trait_fn(self.1, rhs),
                    $trait_name::$trait_fn(self.2, rhs),
                )
            }
        }

        impl $trait_name<$target> for f64 {
            type Output = $target;

            fn $trait_fn(self, rhs: $target) -> Self::Output {
                $target::new(
                    $trait_name::$trait_fn(self, rhs.0),
                    $trait_name::$trait_fn(self, rhs.1),
                    $trait_name::$trait_fn(self, rhs.2),
                )
            }
        }
    };
}

impl_immutable_op!(Vec3, Add, add);
impl_immutable_op!(Vec3, Sub, sub);
impl_immutable_op!(Vec3, Mul, mul);
impl_immutable_op!(Vec3, Div, div);

use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
macro_rules! impl_assign_op {
    ($target:ident, $trait_name:ident, $trait_fn:ident) => {
        impl $trait_name<Self> for $target {
            fn $trait_fn(&mut self, rhs: Self) {
                $trait_name::$trait_fn(&mut self.0, rhs.0);
                $trait_name::$trait_fn(&mut self.1, rhs.1);
                $trait_name::$trait_fn(&mut self.2, rhs.2);
            }
        }
        impl $trait_name<f64> for $target {
            fn $trait_fn(&mut self, rhs: f64) {
                $trait_name::$trait_fn(&mut self.0, rhs);
                $trait_name::$trait_fn(&mut self.1, rhs);
                $trait_name::$trait_fn(&mut self.2, rhs);
            }
        }
    };
}

impl_assign_op!(Vec3, AddAssign, add_assign);
impl_assign_op!(Vec3, SubAssign, sub_assign);
impl_assign_op!(Vec3, MulAssign, mul_assign);
impl_assign_op!(Vec3, DivAssign, div_assign);

use std::ops::{Index, IndexMut, Neg};

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * -1.0
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => panic!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => panic!("Invalid index"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(6.0, 0.0, 1.0);
        let expected_result = Vec3::new(7.0, 2.0, 4.0);
        assert_eq!(v1 + v2, expected_result);
    }

    #[test]
    fn sub_vector() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(6.0, 0.0, 1.0);
        let expected_result = Vec3::new(-5.0, 2.0, 2.0);
        assert_eq!(v1 - v2, expected_result);
    }

    #[test]
    fn add_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 8.0;
        let expected_result = Vec3::new(9.0, 10.0, 11.0);
        assert_eq!(v1 + scalar, expected_result);
    }

    #[test]
    fn add_vector_to_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 8.0;
        let expected_result = Vec3::new(9.0, 10.0, 11.0);
        assert_eq!(scalar + v1, expected_result);
    }

    #[test]
    fn mul_vector_to_scalar() {
        let v1 = Vec3::new(1.0, 2.0, -3.0);
        let scalar = 8.0;
        let expected_result = Vec3::new(8.0, 16.0, -24.0);
        assert_eq!(scalar * v1, expected_result);
    }

    #[test]
    fn sub_scalar() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let scalar = 8.0;
        let expected_result = Vec3::new(-7.0, -6.0, -5.0);
        assert_eq!(v1 - scalar, expected_result);
    }

    #[test]
    fn add_assign_vector() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 += Vec3::new(7.0, 2.0, 4.0);
        let expected_result = Vec3::new(8.0, 4.0, 7.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn sub_assign_vector() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 -= Vec3::new(7.0, 2.0, 4.0);
        let expected_result = Vec3::new(-6.0, 0.0, -1.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn add_assign_scalar() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 += 2.0;
        let expected_result = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn sub_assign_scalar() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 -= 2.0;
        let expected_result = Vec3::new(-1.0, 0.0, 1.0);
        assert_eq!(v1, expected_result);
    }
}
