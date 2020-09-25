use super::{Color, Hittable, HittableList, Point};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    origin: Point,
    direction: Point,
}

impl Ray {
    pub fn new(origin: Point, direction: Point) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point {
        &self.origin
    }

    pub fn direction(&self) -> &Point {
        &self.direction
    }

    pub fn point_at(&self, t: f64) -> Point {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &HittableList, depth: usize) -> Color {
        // (0.00...01 / 2 = 0.00...01) != 0
        const EPS: f64 = 0.000001;
        if depth == 0 {
            Color::black()
        } else if let Some(hit) = world.hit(self, EPS, f64::INFINITY) {
            let target = hit.p + hit.normal + Point::random_unit_vec();
            0.5 * Self::new(hit.p, target - hit.p).color(&world, depth - 1)
        } else {
            let unit_vec = self.direction.unit_vector();
            let t = 0.5 * (unit_vec.y() + 1.0);
            (1.0 - t) * Color::white() + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
