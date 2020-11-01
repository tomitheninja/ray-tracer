#[macro_use]
mod macros;

mod camera;
mod color;
mod config;
mod hit_record;
mod hittable;
mod hittable_list;
mod point;
mod ray;
mod sphere;

use ::rand::random;
use camera::Camera;
use color::Color;
use config::Config;
use hit_record::HitRecord;
use hittable::Hittable;
use hittable_list::HittableList;
use image::{ImageBuffer, Rgb};
use point::Point;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use std::boxed::Box;

fn main() {
    eprintln!("Loading config...");
    let config = Config::from_args();

    let img_width = config.img_width;

    let img_height = config.img_height;

    eprintln!(" width:            {}px", img_width);
    eprintln!(" height:           {}px", img_height);
    eprintln!(" antialias level:  {}", config.samples_per_pixel);
    eprintln!(" ray depth:        {}", config.max_ray_depth);
    eprintln!(" output file:      {}", config.output_file);
    eprintln!();

    // World
    let world = HittableList::default()
        .chain_add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)))
        .chain_add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new(Point::default(), config.aspect_ratio());

    // Render
    let mut bar = progress::Bar::new();
    let mut prev_percent = 0;
    bar.set_job_title("Rendering...");

    let img = ImageBuffer::from_fn(img_width as _, img_height as _, |x, y| {
        let y = img_height as u32 - 1 - y;

        let percent = 100 - 100 * y as i32 / img_height as i32;
        if prev_percent != percent {
            bar.add_percent(percent - prev_percent);
        }
        prev_percent = percent;

        let pixel_color = (0..config.samples_per_pixel)
            .into_par_iter()
            .map(|_| {
                let u = (x as f64 + random::<f64>()) / (img_width as f64 - 1.0);
                let v = (y as f64 + random::<f64>()) / (img_height as f64 - 1.0);

                // Create a ray pointing from the camera to (x, y)
                let ray = camera.get_ray(u, v);

                // Send the ray into the scene
                return ray.color(&world, config.max_ray_depth);
            })
            .reduce(|| Color::black(), |a, b| a + b);

        let color = pixel_color.rgb_bytes(config.samples_per_pixel);

        Rgb([color.0, color.1, color.2])
    });

    img.save(config.output_file).unwrap();
}
