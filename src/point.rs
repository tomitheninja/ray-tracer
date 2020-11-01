#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Point(f64, f64, f64);

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
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

    pub fn random_range(min: f64, max: f64) -> Self {
        use rand::random;
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
        use rand::random;
        use std::f64::consts::PI;
        let r = |min, max| min + (max - min) * random::<f64>();
        let a: f64 = r(0.0, 2.0 * PI);
        let z = r(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Self(r * a.cos(), r * a.sin(), z)
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
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
impl_immutable_op!(Point, Add, add);
impl_immutable_op!(Point, Sub, sub);
impl_immutable_op!(Point, Mul, mul);
impl_immutable_op!(Point, Div, div);
impl_immutable_op!(Point, Neg);
impl_immutable_op!(Point, Index + IndexMut);

impl_assign_op!(Point, AddAssign, add_assign);
impl_assign_op!(Point, SubAssign, sub_assign);
impl_assign_op!(Point, MulAssign, mul_assign);
impl_assign_op!(Point, DivAssign, div_assign);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector() {
        let v1 = Point::new(1.0, 2.0, 3.0);
        let v2 = Point::new(6.0, 0.0, 1.0);
        let expected_result = Point::new(7.0, 2.0, 4.0);
        assert_eq!(v1 + v2, expected_result);
    }

    #[test]
    fn sub_vector() {
        let v1 = Point::new(1.0, 2.0, 3.0);
        let v2 = Point::new(6.0, 0.0, 1.0);
        let expected_result = Point::new(-5.0, 2.0, 2.0);
        assert_eq!(v1 - v2, expected_result);
    }

    #[test]
    fn add_scalar() {
        let v1 = Point::new(1.0, 2.0, 3.0);
        let scalar = 8.0;
        let expected_result = Point::new(9.0, 10.0, 11.0);
        assert_eq!(v1 + scalar, expected_result);
    }

    #[test]
    fn add_vector_to_scalar() {
        let v1 = Point::new(1.0, 2.0, 3.0);
        let scalar = 8.0;
        let expected_result = Point::new(9.0, 10.0, 11.0);
        assert_eq!(scalar + v1, expected_result);
    }

    #[test]
    fn mul_vector_to_scalar() {
        let v1 = Point::new(1.0, 2.0, -3.0);
        let scalar = 8.0;
        let expected_result = Point::new(8.0, 16.0, -24.0);
        assert_eq!(scalar * v1, expected_result);
    }

    #[test]
    fn sub_scalar() {
        let v1 = Point::new(1.0, 2.0, 3.0);
        let scalar = 8.0;
        let expected_result = Point::new(-7.0, -6.0, -5.0);
        assert_eq!(v1 - scalar, expected_result);
    }

    #[test]
    fn add_assign_vector() {
        let mut v1 = Point::new(1.0, 2.0, 3.0);
        v1 += Point::new(7.0, 2.0, 4.0);
        let expected_result = Point::new(8.0, 4.0, 7.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn sub_assign_vector() {
        let mut v1 = Point::new(1.0, 2.0, 3.0);
        v1 -= Point::new(7.0, 2.0, 4.0);
        let expected_result = Point::new(-6.0, 0.0, -1.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn add_assign_scalar() {
        let mut v1 = Point::new(1.0, 2.0, 3.0);
        v1 += 2.0;
        let expected_result = Point::new(3.0, 4.0, 5.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn sub_assign_scalar() {
        let mut v1 = Point::new(1.0, 2.0, 3.0);
        v1 -= 2.0;
        let expected_result = Point::new(-1.0, 0.0, 1.0);
        assert_eq!(v1, expected_result);
    }
}
