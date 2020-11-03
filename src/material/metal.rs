use super::{Material, MaterialResult};
use crate::{Color, HitRecord, Point, Ray};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<MaterialResult> {
        let reflected = r_in.direction().unit_vector().reflect(&rec.normal);
        let scattered = Ray::new(
            rec.position,
            reflected + self.fuzziness * Point::random_in_unit_sphere(),
        );
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
