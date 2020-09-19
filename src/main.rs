mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;

fn main() {
    // Image
    let aspect_ratio: f64 = 16.0 / 9.0;
    let img_width: usize = match std::env::args().nth(1) {
        Some(width) => width.parse().expect("usage: ray_tracer {width}"),
        None => 600,
    };
    let img_height: usize = (img_width as f64 / aspect_ratio) as _;

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = aspect_ratio * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = {
        let half_horizontal: Vec3 = horizontal / 2.0;
        let half_vertical: Vec3 = vertical / 2.0;
        let focal = Vec3::new(0.0, 0.0, focal_length);
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
            let u = (i as f64) / (img_width as f64 - 1.0);
            let v = (j as f64) / (img_height as f64 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            assert_eq!(*ray.origin(), origin);
            let color = ray.color();
            println!("{}", color);
        }
    }

    eprintln!();
    eprintln!("Done.");
}
