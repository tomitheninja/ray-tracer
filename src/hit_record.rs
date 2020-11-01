use super::{Point, Ray};

#[derive(Debug, Default, PartialEq)]
pub struct HitRecord {
    pub position: Point,
    pub normal: Point,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_front_face(&mut self, ray: &Ray, outward_normal: &Point) {
        self.front_face = Point::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}
