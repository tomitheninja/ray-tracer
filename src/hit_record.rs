use super::{Point, Ray};
use crate::material::Material;
use std::{fmt::Debug, sync::Arc};

#[derive(Debug)]
pub struct HitRecord {
    pub position: Point,
    pub normal: Point,
    pub material: Arc<dyn Material>,
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
