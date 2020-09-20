mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Color, Point};

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = match std::env::args().nth(1) {
        Some(width) => width.parse().expect("Usage: ray_tracing {width}"),
        None => 600,
    };
    let img_height = (img_width as f64 / aspect_ratio) as _;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Point::new(viewport_width, 0.0, 0.0);
    let vertical = Point::new(0.0, viewport_height, 0.0);
    let lower_left_corner = {
        let half_horizontal = horizontal / 2.0;
        let half_vertical = vertical / 2.0;
        let focal = Point::new(0.0, 0.0, focal_length);
        origin - half_horizontal - half_vertical - focal
    };

    // Render
    println!("P3");
    println!("{width} {height}", width = img_width, height = img_height);
    println!("{color_depth}", color_depth = 255);

    for j in (0..img_height).rev() {
        let steps = (img_height - 1) / 25.min(img_height - 1);
        if j % steps == 0 {
            eprint!("\rScanlines remaining: {:>4}", j);
        }

        for i in 0..img_width {
            let u = i as f64 / (img_width as f64 - 1.0);
            let v = j as f64 / (img_height as f64 - 1.0);

            let direction = lower_left_corner + u * horizontal + v * vertical - origin;
            let ray = Ray::new(&origin, &direction);
            let color = ray.color(&world).rgb();
            println!("{}", color);
        }
    }

    eprintln!();
    eprintln!("Done.");
}
