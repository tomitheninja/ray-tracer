use super::{HitRecord, Hittable, Point, Ray};

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
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // A sphere is given using x^2 + y^2 + z^2 = r^2
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
            let root = discriminant.sqrt();

            for &signed_root in [root, -root].iter() {
                let temp = (-half_b - signed_root) / a;

                if t_min < temp && temp < t_max {
                    let t = temp;
                    let p = r.point_at(temp);
                    let outward_normal = (p - self.center) / self.radius;
                    let mut result = HitRecord {
                        t,
                        p,
                        ..HitRecord::default()
                    };
                    result.set_front_face(r, &outward_normal);
                    return Some(result);
                }
            }
        }
        None
    }
}
