mod camera;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use ::rand::random;
use camera::Camera;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Color, Point};

const USAGE: &'static str = "Usage: ray_tracing {width?} {aa-samples?} {max_ray_depth?}";

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let img_width: usize = match std::env::args().nth(1) {
        Some(width) => width.parse().expect(USAGE),
        None => 600,
    };
    let img_height = (img_width as f64 / aspect_ratio) as _;
    let samples_per_pixel: usize = match std::env::args().nth(2) {
        Some(samples) => samples.parse().expect(USAGE),
        None => 100,
    };
    let max_depth: usize = match std::env::args().nth(3) {
        Some(samples) => samples.parse().expect(USAGE),
        None => 50,
    };

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{width} {height}", width = img_width, height = img_height);
    println!("{color_depth}", color_depth = 255);

    for j in (0..img_height).rev() {
        let steps = (img_height - 1) / 100.min(img_height - 1);
        if j % steps == 0 {
            eprint!("\rScanlines remaining: {:>4} / {}", j, img_height - 1);
        }

        for i in 0..img_width {
            let mut pixel_color = Color::default();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random::<f64>()) / (img_width as f64 - 1.0);
                let v = (j as f64 + random::<f64>()) / (img_height as f64 - 1.0);
                let ray = camera.get_ray(u, v);
                pixel_color += ray.color(&world, max_depth);
            }

            let color = pixel_color.rgb(samples_per_pixel);
            println!("{}", color);
        }
    }

    eprintln!();
    eprintln!("Done.");
}
