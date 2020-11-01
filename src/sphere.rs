use super::{HitRecord, Hittable, Point, Ray};

/// Sphere's body can be calculated
/// using the `x^2 + y^2 + z^2 <= r^2` formula
#[derive(Debug, Default, PartialEq)]
pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    /// It is easy to calculate if a ray hits the sphere, because
    ///
    /// **if the discriminant is**
    ///
    /// - greater than 0,
    /// then there are two intersections (front and back).
    ///
    /// - smaller than or equal to 0,
    /// then the ray missed the object
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // A sphere is given using x^2 + y^2 + z^2 <= r^2
        //
        // If the ray goes through the sphere
        // then the discriminant > 0 (there are two points where it hits the sphere)
        //
        // If the ray hits the edge of the sphere
        // then the discriminant = 0 (there is exactly one point)
        //
        // If the ray missed
        // then the discriminant < 0 (there are no solutions)
        let oc = *r.origin() - self.center;
        let a = r.direction().len_squared();
        let half_b = Point::dot(&oc, &r.direction());
        let c = oc.len_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            let discriminant = discriminant.sqrt();

            // Try both intersections
            for &signed_root in [discriminant, -discriminant].iter() {
                // (-b +- sqrt(d)) / 2a
                let t = (-half_b - signed_root) / a;

                // The front one will be between t_min and t_max
                if t_min < t && t < t_max {
                    let position = r.point_at(t).unwrap();
                    let outward_normal = (position - self.center) / self.radius;
                    let mut result = HitRecord {
                        t,
                        position,
                        ..HitRecord::default()
                    };
                    result.set_front_face(r, &outward_normal);
                    return Some(result);
                }
            }
        }
        None // All you had to do is follow the damn sphere Cray!
    }
}
