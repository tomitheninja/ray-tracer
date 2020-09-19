use super::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Vec3 {
        *self.origin() + t * *self.direction()
    }

    pub fn color(&self) -> Vec3 {
        let unit_dir = self.direction().unit_vector();
        let t = 0.5 * (unit_dir.y() + 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        let color2 = Vec3::new(0.5, 0.7, 1.0);
        (1.0 - t) * white + t * color2
    }
}
