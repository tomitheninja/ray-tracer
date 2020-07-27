use super::vec3::Vec3;
use std::ops::Deref;

struct Point3 {
    data: Vec3<f64>,
}

impl Deref for Point3 {
    type Target = Vec3<f64>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl Point3 {
    pub fn x(&self) -> f64 {
        self[0]
    }

    pub fn y(&self) -> f64 {
        self[1]
    }

    pub fn z(&self) -> f64 {
        self[2]
    }
}
