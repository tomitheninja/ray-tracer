mod color;
mod point3;
mod vec3;

use color::Color;

fn main() {
    // Image

    const IMG_WIDTH: usize = 256;
    const IMG_HEIGHT: usize = 256;

    // Render
    println!("P3");
    println!("{width} {height}", width = IMG_WIDTH, height = IMG_HEIGHT);
    println!("{color_depth}", color_depth = 255);

    for j in (0..IMG_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..IMG_WIDTH {
            let r = (i as f64) / (IMG_WIDTH - 1) as f64;
            let g = (j as f64) / (IMG_HEIGHT - 1) as f64;
            let b = 0.25;

            println!("{}", Color::new(r, g, b));
        }
    }

    eprintln!();
    eprintln!("Done.");
}
