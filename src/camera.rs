use super::{Point, Ray};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Point,
    vertical: Point,
}

impl Camera {
    pub fn new(origin: Point, aspect_ratio: f64) -> Self {
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let horizontal = Point::new(viewport_width, 0.0, 0.0);
        let vertical = Point::new(0.0, viewport_height, 0.0);
        let lower_left_corner = {
            let half_horizontal = horizontal / 2.0;
            let half_vertical = vertical / 2.0;
            let focal = Point::new(0.0, 0.0, focal_length);
            origin - half_horizontal - half_vertical - focal
        };

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_direction(&self, u: f64, v: f64) -> Point {
        self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin
    }

    /// Create a ray which points from the camera to he real (x, y)
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let direction = self.get_direction(u, v);
        Ray::new(self.origin, direction)
    }
}
