#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self(r, g, b)
    }

    /// Get colors encoded as RGB bytes
    pub fn rgb_bytes(&self, samples_per_pixel: usize) -> (u8, u8, u8) {
        use std::u8::{MAX, MIN};
        let f64_to_u8 = |x: f64| x.min(MAX as f64).max(MIN as f64) as u8;

        let scale = 1.0 / (samples_per_pixel as f64);

        return (
            f64_to_u8(256.0 * (self.0 * scale).sqrt()),
            f64_to_u8(256.0 * (self.1 * scale).sqrt()),
            f64_to_u8(256.0 * (self.2 * scale).sqrt()),
        );
    }

    /// Get colors encoded as space separated RGB stringified bytes
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

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
impl_immutable_op!(Color, Add, add);
impl_immutable_op!(Color, Sub, sub);
impl_immutable_op!(Color, Mul, mul);
impl_immutable_op!(Color, Div, div);
impl_immutable_op!(Color, Neg);
impl_immutable_op!(Color, Index + IndexMut);

impl_assign_op!(Color, AddAssign, add_assign);
impl_assign_op!(Color, SubAssign, sub_assign);
impl_assign_op!(Color, MulAssign, mul_assign);
impl_assign_op!(Color, DivAssign, div_assign);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black() {
        assert_eq!((0, 0, 0), Color::black().rgb_bytes(1));
    }

    #[test]
    fn test_white() {
        assert_eq!((255, 255, 255), Color::white().rgb_bytes(1));
    }
}
