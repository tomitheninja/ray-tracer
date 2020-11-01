use super::{HitRecord, Ray};
use std::fmt::Debug;

/// An object that the ray can hit
pub trait Hittable: Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
