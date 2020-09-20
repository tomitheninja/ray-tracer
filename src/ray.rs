use super::{Color, Point};

#[derive(Debug, Copy, Clone)]
pub struct Ray<'a> {
    origin: &'a Point,
    direction: &'a Point,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: &'a Point) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        self.origin
    }

    pub fn direction(&self) -> &Point {
        self.direction
    }

    pub fn point_at(&self, t: f64) -> Point {
        *self.origin + t * *self.direction
    }

    pub fn color(&self) -> Color {
        if let Some(t) = self.hit_sphere(&Point::new(0.0, 0.0, -1.0), 0.5) {
            let n = self.point_at(t) - Point::new(0.0, 0.0, -1.0);
            0.5 * (n.unit_vector() + 1.0)
        } else {
            let unit_vec = self.direction.unit_vector();
            let t = 0.5 * (unit_vec.y() + 1.0);
            (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
        }
    }

    pub fn hit_sphere(&self, center: &Point, r: f64) -> Option<f64> {
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
        let oc = *self.origin - *center;
        let a = Point::dot(&self.direction, &self.direction);
        let b = 2.0 * Point::dot(&oc, &self.direction);
        let c = Point::dot(&oc, &oc) - r.powi(2);
        let discriminant = b.powi(2) - 4.0 * a * c;
        if discriminant < 0.0 {
            None
        } else {
            Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}
