use super::{Color, Hittable, HittableList, Point};

/// Create a ray that goes from origin to infinity in a given direction
#[derive(Debug, Default, Copy, Clone)]
pub struct Ray {
    origin: Point,
    direction: Point,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RayError {
    InvalidDirection,
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

    /// This method returns the current location of the ray if it wouldn't hit a sphere
    /// (maybe it really didn't hit it)
    ///
    /// Please not the parameter is only named time because
    /// it behaves like time in physic's motion
    ///
    /// When writing a ray tracer every point
    /// between origin and infinity
    /// should be tested for collision against all available objects
    /// to find out if it hit any (and hence the ray is blocked)
    pub fn point_at(&self, time: f64) -> Result<Point, RayError> {
        // Only real position is supported
        // Ray should never go backwards
        if time >= 0.0 {
            Ok(self.origin + time * self.direction)
        } else {
            Err(RayError::InvalidDirection)
        }
    }

    /// Calculate the resulting color of the ray
    ///
    /// **If the ray ...**
    /// - hits an object
    ///
    /// It will take some of it's color
    /// and a weaker new ray will continue on approximately the hit's normal vector
    ///
    /// - gets stuck between two objects
    ///
    /// It will slowly fade away
    ///
    /// - does not hit anything
    ///
    /// It will simulate the color of the sky
    pub fn color(&self, world: &HittableList, allowed_collisions: usize) -> Color {
        // The function is called recursively
        // The start of each iteration = hit point + 0.0000*t
        // The direction of the new ray is the normal vector of the object + some random (anti aliased)
        if allowed_collisions == 0 {
            // Stuck in a mirror room
            // The ray will fade away here
            return Color::black();
        } else if let Some(hit) = world.hit(&self, f64::EPSILON, f64::INFINITY) {
            // Hit an object
            if let Some(mat) = hit.material.scatter(&self, &hit) {
                mat.attenuation * mat.scattered.color(&world, allowed_collisions - 1)
            } else {
                Color::black()
            }

        // return 0.5 * new_ray.color(&world, allowed_collisions - 1);
        } else {
            // Reached Infinity
            // Let's give the sky a nice gradient color
            // based on the y coordinate
            // t=0 => start_value
            // t=1 => end_value
            let unit_vec = self.direction.unit_vector();
            let t = 0.5 * (unit_vec.y() + 1.0);
            let start_value = Color::white();
            let end_value = Color::new(0.5, 0.7, 1.0);
            return (1.0 - t) * start_value + t * end_value;
        }
    }
}
