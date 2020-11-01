use super::{Material, MaterialResult};
use crate::{Color, HitRecord, Point, Ray};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(rec.position, reflected);
        let attenuation = self.albedo;
        if Point::dot(&scattered.direction(), &rec.normal) > 0.0 {
            Some(MaterialResult {
                scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}
