use super::{Color, Point3};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Ray {
    origin: Point3,
    direction: Point3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Point3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Point3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        *self.origin() + t * *self.direction()
    }

    pub fn color(&self) -> Color {
        let t = self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = self.at(t) - Point3::new(0.0, 0.0, -1.0);
            0.5 * (n + 1.0)
        } else {
            let unit_dir = self.direction().unit_vector();
            let t = 0.5 * (unit_dir.y() + 1.0);
            let white = Color::new(1.0, 1.0, 1.0);
            let color2 = Color::new(0.5, 0.7, 1.0);
            (1.0 - t) * white + t * color2
        }
    }

    /// A sphere is stored as `x^2 + y^2 + z^2 = r^2`
    ///
    /// If the quadratic equation has two solutions (disc > 0)
    ///
    /// The the ray goes through the sphere
    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> f64 {
        let oc = *self.origin() - *center;
        let a = self.direction().len_squared();
        let half_b = oc.dot(self.direction());
        let c = oc.len_squared() - radius * radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            -1.0
        } else {
            (half_b + discriminant.sqrt()) / a
        }
    }
}
