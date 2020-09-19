use std::fmt;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Vec3(f64, f64, f64);

impl Vec3 {
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

    pub fn len_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * self.z()
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self(
            self.y() * other.z() - self.z() * other.y(),
            self.z() * other.x() - self.x() * other.z(),
            self.x() * other.y() - self.y() * other.x(),
        )
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }

    pub fn rgb(&self) -> (u8, u8, u8) {
        let as_u8 = |x: f64| ((x * 255.999) as u8);
        (as_u8(self.x()), as_u8(self.y()), as_u8(self.z()))
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (r, g, b) = self.rgb();
        write!(f, "{} {} {}", r, g, b)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.0, -self.1, -self.2)
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => unreachable!("Invalid index"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            _ => unreachable!("Invalid index"),
        }
    }
}

macro_rules! gen_immutable_ops {
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
                self + Self::new(rhs, rhs, rhs)
            }
        }

        impl $trait_name<$target> for f64 {
            type Output = $target;

            fn $trait_fn(self, rhs: $target) -> Self::Output {
                rhs + self
            }
        }
    };
}

gen_immutable_ops!(Vec3, Add, add);
gen_immutable_ops!(Vec3, Sub, sub);
gen_immutable_ops!(Vec3, Div, div);
gen_immutable_ops!(Vec3, Mul, mul);

macro_rules! gen_assign_ops {
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

gen_assign_ops!(Vec3, AddAssign, add_assign);
gen_assign_ops!(Vec3, SubAssign, sub_assign);
gen_assign_ops!(Vec3, DivAssign, div_assign);
gen_assign_ops!(Vec3, MulAssign, mul_assign);

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
    fn add_assign_vector() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 += Vec3::new(7.0, 2.0, 4.0);
        let expected_result = Vec3::new(8.0, 4.0, 7.0);
        assert_eq!(v1, expected_result);
    }

    #[test]
    fn add_assign_scalar() {
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        v1 += 2.0;
        let expected_result = Vec3::new(3.0, 4.0, 5.0);
        assert_eq!(v1, expected_result);
    }
}
