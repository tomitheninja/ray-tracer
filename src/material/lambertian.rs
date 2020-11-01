use super::{Material, MaterialResult};
use crate::{Color, HitRecord, Point, Ray};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let mut scatter_direction = rec.normal + Point::random_unit_vec();

        // Prevent NaN issues
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal
        }

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.position, scatter_direction);

        Some(MaterialResult {
            attenuation,
            scattered,
        })
    }
}
