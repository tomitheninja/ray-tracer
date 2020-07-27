use super::vec3::Vec3;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::ops::Deref;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    data: Vec3<f64>,
}

impl Deref for Color {
    type Target = Vec3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Color {
    pub fn r(&self) -> f64 {
        self[0]
    }

    pub fn r_byte(&self) -> u8 {
        (self.r() * 255.99) as _
    }

    pub fn g(&self) -> f64 {
        self[1]
    }

    pub fn g_byte(&self) -> u8 {
        (self.g() * 255.99) as _
    }

    pub fn b(&self) -> f64 {
        self[2]
    }

    pub fn b_byte(&self) -> u8 {
        (self.b() * 255.99) as _
    }

    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self {
            data: Vec3::new(r, g, b),
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{} {} {}", self.r_byte(), self.g_byte(), self.b_byte())
    }
}
